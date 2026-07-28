//! Python bindings for the Coral runtime. A small pycoral-style surface:
//! `make_interpreter(model)` → an interpreter with `set_input`/`invoke`/
//! `get_output`, plus `flash(firmware)` for a device in DFU mode. The runtime
//! drives the accelerator directly — no libedgetpu, no TFLite.

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

fn err(context: &str, e: impl std::fmt::Display) -> PyErr {
    PyRuntimeError::new_err(format!("{context}: {e}"))
}

#[pyclass]
struct Interpreter {
    program: ::cay::program::Program,
    input: Vec<u8>,
    output: Vec<u8>,
}

#[pymethods]
impl Interpreter {
    #[new]
    fn new(model_path: &str) -> PyResult<Self> {
        let model = std::fs::read(model_path).map_err(|e| err("read model", e))?;
        let pkg = ::cay::program::extract_package(&model)
            .ok_or_else(|| PyRuntimeError::new_err("no DWN1 package in model"))?;
        let program = ::cay::program::Program::from_package(pkg).map_err(|e| err("parse model", e))?;
        Ok(Interpreter {
            program,
            input: Vec::new(),
            output: Vec::new(),
        })
    }

    fn set_input(&mut self, data: &[u8]) {
        self.input = data.to_vec();
    }

    /// Brings the chip up and runs one inference over the current input. Output
    /// tensors are concatenated in program order (one for a classifier, raw heads
    /// for detection).
    fn invoke(&mut self) -> PyResult<()> {
        let driver = ::cay::driver::Driver::open().map_err(|e| err("chip bring-up", e))?;
        driver.run().map_err(|e| err("run", e))?;
        self.output = driver
            .run_program(&self.program, &[&self.input])
            .map_err(|e| err("inference", e))?
            .concat();
        Ok(())
    }

    fn get_output<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound(py, &self.output)
    }

    #[getter]
    fn input_size(&self) -> usize {
        self.program.inputs.iter().map(|i| i.bytes).sum()
    }

    #[getter]
    fn output_size(&self) -> usize {
        self.program.outputs.iter().map(|o| o.bytes).sum()
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
