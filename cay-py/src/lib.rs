//! Python bindings for the cay runtime.
//!
//! The surface mirrors `tflite_runtime.interpreter.Interpreter` —
//! `allocate_tensors`, `get_input_details`, `get_output_details`, `set_tensor`,
//! `invoke`, `get_tensor` — so code written against the Edge TPU delegate runs
//! here with an import swap.
//!
//! This runs the Edge TPU program embedded in the model and returns that
//! program's output tensors. Operators the compiler left on the CPU, such as
//! `TFLite_Detection_PostProcess` in stock SSD graphs, do not execute, so those
//! graphs yield raw heads rather than decoded boxes. Post-process in the caller.
//!
//! Arrays go through numpy's Python API rather than rust-numpy, which keeps the
//! extension on the limited API: one abi3 wheel per platform covers CPython 3.8
//! up.

use pyo3::exceptions::{PyIndexError, PyRuntimeError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList};

fn err(context: &str, e: impl std::fmt::Display) -> PyErr {
    PyRuntimeError::new_err(format!("{context}: {e}"))
}

/// numpy scalar type for a tensor, from its element width and signedness.
fn dtype_name(scalar_bytes: usize, signed: bool) -> &'static str {
    match (scalar_bytes, signed) {
        (1, true) => "int8",
        (1, false) => "uint8",
        (2, true) => "int16",
        (2, false) => "uint16",
        (4, true) => "int32",
        _ => "uint8",
    }
}

fn numpy_dtype<'py>(py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
    py.import_bound("numpy")?.getattr(name)
}

#[pyclass]
struct Interpreter {
    program: ::cay::program::Program,
    /// Staged input buffers, one per `program.inputs`, parallel by position.
    inputs: Vec<Vec<u8>>,
    /// Filled by `invoke`, one per `program.outputs`.
    outputs: Vec<Vec<u8>>,
    /// Positions in `program.outputs` that are user-visible, in order. Recurrent
    /// state buffers are written by the accelerator but are not model outputs.
    visible: Vec<usize>,
    /// Opened on the first `invoke` and held: bring-up costs an order of
    /// magnitude more than an inference, and a held driver keeps the model's
    /// weights resident on-chip between calls.
    driver: Option<::cay::driver::Driver>,
}

#[pymethods]
impl Interpreter {
    #[new]
    fn new(model_path: &str) -> PyResult<Self> {
        let model = std::fs::read(model_path).map_err(|e| err("read model", e))?;
        let pkg = ::cay::program::extract_package(&model)
            .ok_or_else(|| PyRuntimeError::new_err("no DWN1 package in model"))?;
        let program =
            ::cay::program::Program::from_package(pkg).map_err(|e| err("parse model", e))?;
        let visible = program
            .outputs
            .iter()
            .enumerate()
            .filter(|(_, o)| !o.variable)
            .map(|(i, _)| i)
            .collect();
        let inputs = program.inputs.iter().map(|i| vec![0u8; i.bytes]).collect();
        Ok(Interpreter {
            program,
            inputs,
            outputs: Vec::new(),
            visible,
            driver: None,
        })
    }

    /// Present for API compatibility. cay sizes its buffers when the model is
    /// parsed and brings the device up inside `invoke`, so nothing is allocated
    /// here.
    fn allocate_tensors(&self) {}

    /// Tensor indices are positional: inputs occupy `0..n`, outputs continue
    /// from `n`. They are opaque handles to hand back to `set_tensor` and
    /// `get_tensor`, not TFLite graph indices.
    fn get_input_details<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let mut rows = Vec::with_capacity(self.program.inputs.len());
        for (i, spec) in self.program.inputs.iter().enumerate() {
            let d = PyDict::new_bound(py);
            d.set_item("index", i)?;
            d.set_item("name", &spec.name)?;
            // The executable records the DMA buffer size, not the source
            // tensor's spatial dims, so the shape is reported flat.
            d.set_item("shape", vec![1usize, spec.bytes / spec.scalar_bytes.max(1)])?;
            d.set_item(
                "dtype",
                numpy_dtype(py, dtype_name(spec.scalar_bytes, spec.signed))?,
            )?;
            d.set_item("quantization", (spec.scale, spec.zero_point))?;
            rows.push(d);
        }
        Ok(PyList::new_bound(py, rows))
    }

    /// Shapes are the accelerator's `[batch, H, W, C]` view of what the TPU
    /// program writes. Where the graph trims a dimension on the CPU afterwards
    /// — SSD's `Squeeze` before the box decode, say — tflite reports the
    /// trimmed rank and cay reports the untrimmed one, because cay stops at the
    /// accelerator. Same element count, one more axis.
    fn get_output_details<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let base = self.program.inputs.len();
        let mut rows = Vec::with_capacity(self.visible.len());
        for (slot, &pos) in self.visible.iter().enumerate() {
            let spec = &self.program.outputs[pos];
            let d = PyDict::new_bound(py);
            d.set_item("index", base + slot)?;
            d.set_item("name", &spec.name)?;
            let [h, w, c] = spec.dims;
            d.set_item("shape", vec![1usize, h, w, c])?;
            d.set_item(
                "dtype",
                numpy_dtype(py, dtype_name(spec.scalar_bytes, spec.signed))?,
            )?;
            d.set_item("quantization", (spec.scale, spec.zero_point))?;
            rows.push(d);
        }
        Ok(PyList::new_bound(py, rows))
    }

    /// Accepts any object exposing `tobytes` (a numpy array) or raw bytes.
    fn set_tensor(&mut self, index: usize, value: &Bound<'_, PyAny>) -> PyResult<()> {
        if index >= self.inputs.len() {
            return Err(PyIndexError::new_err(format!(
                "input index {index} out of range; model has {} input(s)",
                self.inputs.len()
            )));
        }
        let data: Vec<u8> = match value.call_method0("tobytes") {
            Ok(b) => b.extract()?,
            Err(_) => value.extract()?,
        };
        let want = self.program.inputs[index].bytes;
        if data.len() != want {
            return Err(PyRuntimeError::new_err(format!(
                "input {index} expects {want} bytes, got {}",
                data.len()
            )));
        }
        self.inputs[index] = data;
        Ok(())
    }

    /// Runs one inference over the staged inputs, bringing the chip up on the
    /// first call.
    fn invoke(&mut self) -> PyResult<()> {
        if self.driver.is_none() {
            let d = ::cay::driver::Driver::open().map_err(|e| err("chip bring-up", e))?;
            d.run().map_err(|e| err("run", e))?;
            self.driver = Some(d);
        }
        let refs: Vec<&[u8]> = self.inputs.iter().map(|v| v.as_slice()).collect();
        let driver = self.driver.as_ref().expect("opened above");
        match driver.run_program(&self.program, &refs) {
            Ok(outputs) => {
                self.outputs = outputs;
                Ok(())
            }
            Err(e) => {
                // run_program's recovery can undo what open() set up, so the
                // next invoke starts from a fresh handle.
                self.driver = None;
                Err(err("inference", e))
            }
        }
    }

    /// The tensor at `index` as a numpy array of its declared shape and dtype.
    /// Values are the raw quantized output; apply `quantization` yourself, the
    /// same as with the TFLite interpreter.
    fn get_tensor<'py>(&self, py: Python<'py>, index: usize) -> PyResult<Bound<'py, PyAny>> {
        let base = self.program.inputs.len();
        let Some(slot) = index.checked_sub(base).filter(|s| *s < self.visible.len()) else {
            return Err(PyIndexError::new_err(format!(
                "output index {index} out of range; outputs occupy {base}..{}",
                base + self.visible.len()
            )));
        };
        if self.outputs.is_empty() {
            return Err(PyRuntimeError::new_err("call invoke() before get_tensor()"));
        }
        let pos = self.visible[slot];
        let spec = &self.program.outputs[pos];
        let raw = PyBytes::new_bound(py, &self.outputs[pos]);
        let np = py.import_bound("numpy")?;
        let dtype = numpy_dtype(py, dtype_name(spec.scalar_bytes, spec.signed))?;
        let flat = np.call_method1("frombuffer", (raw, dtype))?;
        let [h, w, c] = spec.dims;
        flat.call_method1("reshape", ((1usize, h, w, c),))
    }

    /// Halts the chip and releases it without waiting for collection. Calling
    /// it twice is harmless; a later `invoke` reopens the device.
    fn close(&mut self) {
        self.driver = None;
    }

    /// Single-input convenience retained from the original surface.
    fn set_input(&mut self, data: &[u8]) -> PyResult<()> {
        if self.inputs.is_empty() {
            return Err(PyRuntimeError::new_err("model declares no inputs"));
        }
        self.inputs[0] = data.to_vec();
        Ok(())
    }

    /// Every visible output concatenated in program order.
    fn get_output<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        let mut all = Vec::new();
        for &pos in &self.visible {
            if let Some(buf) = self.outputs.get(pos) {
                all.extend_from_slice(buf);
            }
        }
        PyBytes::new_bound(py, &all)
    }

    #[getter]
    fn input_size(&self) -> usize {
        self.program.inputs.iter().map(|i| i.bytes).sum()
    }

    #[getter]
    fn output_size(&self) -> usize {
        self.visible
            .iter()
            .map(|&p| self.program.outputs[p].bytes)
            .sum()
    }
}

#[pyfunction]
fn make_interpreter(model_path: &str) -> PyResult<Interpreter> {
    Interpreter::new(model_path)
}

/// Flashes the Apex firmware onto a device in DFU mode so it enumerates in
/// runtime mode.
#[pyfunction]
fn flash(firmware_path: &str) -> PyResult<()> {
    let fw = std::fs::read(firmware_path).map_err(|e| err("read firmware", e))?;
    ::cay::dfu::flash(&fw).map_err(|e| err("flash", e))
}

#[pymodule]
fn cay(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Interpreter>()?;
    m.add_function(wrap_pyfunction!(make_interpreter, m)?)?;
    m.add_function(wrap_pyfunction!(flash, m)?)?;
    Ok(())
}
