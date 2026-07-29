//! Dumps chip state right after open, so a working open can be diffed against
//! one that is about to fail.
//!
//! Run it after a successful inference of one model, then after a successful
//! inference of another; whatever differs is what the next inference inherits.

use cay::csr;
use cay::ml::Csr;

fn main() {
    let csr = Csr::open().expect("open");
    let names: &[(&str, u32)] = &[
        ("SCU_CTRL_0", csr::SCU_CTRL_0),
        ("SCU_CTRL_2", csr::SCU_CTRL_2),
        ("SCU_CTRL_3", csr::SCU_CTRL_3),
        ("SCU_CTR_7", csr::SCU_CTR_7),
        ("OMC0_00", csr::OMC0_00),
        ("OMC0_D4", csr::OMC0_D4),
        ("OMC0_D8", csr::OMC0_D8),
        ("RAMBIST_CTRL_1", csr::RAMBIST_CTRL_1),
        ("SLV_ABM_EN", csr::SLV_ABM_EN),
        ("MST_ABM_EN", csr::MST_ABM_EN),
        ("SLV_ERR_RESP_ISR_MASK", csr::SLV_ERR_RESP_ISR_MASK),
        ("MST_ERR_RESP_ISR_MASK", csr::MST_ERR_RESP_ISR_MASK),
        ("GCBB_CREDIT0", csr::GCBB_CREDIT0),
    ];
    for (name, offset) in names {
        match csr.read32(*offset) {
            Ok(value) => println!("{name:24} {offset:#08x} {value:#010x}"),
            Err(e) => println!("{name:24} {offset:#08x} read failed: {e}"),
        }
    }

    let wide: &[(&str, u32)] = &[
        ("SCALAR_CORE_RUN_CONTROL", csr::SCALAR_CORE_RUN_CONTROL),
        ("IDLE_REGISTER", csr::IDLE_REGISTER),
        ("TILECONFIG0", csr::TILECONFIG0),
        ("DEEP_SLEEP", csr::DEEP_SLEEP),
        ("DESCR_EP", csr::DESCR_EP),
        ("MULTI_BO_EP", csr::MULTI_BO_EP),
        ("OUTFEED_CHUNK_LENGTH", csr::OUTFEED_CHUNK_LENGTH),
        ("HIB_ERROR_STATUS", csr::HIB_ERROR_STATUS),
        ("HIB_FIRST_ERROR_STATUS", csr::HIB_FIRST_ERROR_STATUS),
        ("FATAL_ERR_INT_CONTROL", csr::FATAL_ERR_INT_CONTROL),
    ];
    for (name, offset) in wide {
        match csr.read64(*offset) {
            Ok(value) => println!("{name:24} {offset:#08x} {value:#018x}"),
            Err(e) => println!("{name:24} {offset:#08x} read failed: {e}"),
        }
    }
    for (i, offset) in csr::SCALAR_CORE_RUN_CONTROLS.iter().enumerate() {
        match csr.read64(*offset) {
            Ok(value) => println!("scalar_run[{i}]           {offset:#08x} {value:#018x}"),
            Err(e) => println!("scalar_run[{i}]           {offset:#08x} read failed: {e}"),
        }
    }
    for (i, offset) in csr::TOP_LEVEL_INT_CONTROL.iter().enumerate() {
        match csr.read64(*offset) {
            Ok(value) => println!("top_level_int[{i}]        {offset:#08x} {value:#018x}"),
            Err(e) => println!("top_level_int[{i}]        {offset:#08x} read failed: {e}"),
        }
    }

    // The USB block, where descriptor and DMA state lives. Named constants only
    // cover the handful the driver writes; whatever carries a model's residency
    // across a close is somewhere in here.
    for offset in (0x4c000..0x4c200).step_by(8) {
        if let Ok(value) = csr.read64(offset) {
            if value != 0 {
                println!("usb                      {offset:#08x} {value:#018x}");
            }
        }
    }
}
