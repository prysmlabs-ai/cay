//! coral-reset — issues a USB port reset to the runtime-mode device. Recovers a
//! Coral wedged badly enough that CSR control transfers time out (e.g. after an
//! aborted inference), without needing a physical replug.

use cay::ml::Csr;

fn main() {
    let csr = match Csr::open() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("open runtime device: {e}");
            std::process::exit(1);
        }
    };
    match csr.reset() {
        Ok(()) => eprintln!("device reset"),
        Err(e) => {
            eprintln!("reset: {e}");
            std::process::exit(1);
        }
    }
}
