//! Dumps the raw DarwiNN structure of a compiled edge-TPU model: each
//! executable's type, input/output layers, and DMA hints in order.

use std::fs;

fn main() {
    let path = std::env::args().nth(1).expect("usage: dump <model.tflite>");
    let model = fs::read(&path).expect("read model");
    dump(&model).expect("dump");
}

fn dump(model: &[u8]) -> cay_program::Result<()> {
    let pkg = cay_program::extract_package(model).expect("no DWN1 package");
    let parsed = cay_program::parse_package(pkg)?;
    let multi = cay_program::multi_executable_bytes(&parsed)?;
    for (bi, blob) in cay_program::executable_blobs(multi)?.iter().enumerate() {
        let exec = cay_program::parse_executable(blob)?;
        println!(
            "EXEC {bi} type={:?} token=0x{:x} params={}",
            exec.type_()?,
            exec.parameter_caching_token()?,
            exec.parameters()?.map_or(0, |p| p.len()),
        );
        for (kind, layers) in [("in", exec.input_layers()?), ("out", exec.output_layers()?)] {
            let Some(ls) = layers else { continue };
            for l in ls.iter() {
                let l = l?;
                println!(
                    "  {kind} name={:?} x={} y={} z={} dt={:?} exec_count={}",
                    l.name()?,
                    l.x_dim()?,
                    l.y_dim()?,
                    l.z_dim()?,
                    l.data_type()?,
                    l.execution_count_per_inference()?
                );
            }
        }
        let Some(hints) = exec.dma_hints()? else {
            continue;
        };
        let Some(hs) = hints.hints()? else {
            continue;
        };
        for h in hs.iter() {
            let h = h?;
            let dir = h.direction()?;
            match h.any_hint()? {
                Some(cay_program::schema::AnyHintRef::DmaDescriptorHint(d)) => {
                    let (desc, name) = match d.meta()? {
                        Some(m) => (Some(m.desc()?), m.name()?),
                        None => (None, None),
                    };
                    println!(
                        "    DMA {dir:?} desc={desc:?} name={name:?} off={} size={}",
                        d.offset_in_bytes()?,
                        d.size_in_bytes()?
                    );
                }
                Some(cay_program::schema::AnyHintRef::InstructionHint(i)) => {
                    println!("    INSTR chunk={}", i.instruction_chunk_index()?);
                }
                Some(cay_program::schema::AnyHintRef::InterruptHint(_)) => {
                    println!("    INTERRUPT")
                }
                Some(cay_program::schema::AnyHintRef::FenceHint(_)) => println!("    FENCE"),
                None => {}
            }
        }
    }
    Ok(())
}
