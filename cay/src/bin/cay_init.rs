fn main() {
    let driver = match cay::driver::Driver::open() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("chip init failed: {e}");
            std::process::exit(1);
        }
    };

    match driver.scalar_core_state() {
        Ok(v) => println!("after init: scalarCoreRunControl = {v} (readable; stalled before init)"),
        Err(e) => {
            eprintln!("read scalar core: {e}");
            std::process::exit(1);
        }
    }

    if let Err(e) = driver.run() {
        eprintln!("run trigger failed: {e}");
        std::process::exit(1);
    }
    match driver.scalar_core_state() {
        Ok(v) => println!("after run:  scalarCoreRunControl = {v} (expect 1 = running)"),
        Err(e) => {
            eprintln!("read scalar core: {e}");
            std::process::exit(1);
        }
    }

    if let Err(e) = driver.enable_interrupts() {
        eprintln!("enable interrupts: {e}");
        std::process::exit(1);
    }
    println!("chip up.");
}
