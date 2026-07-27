use std::time::Duration;

fn report(label: &str) {
    match cay::usb::find_coral() {
        Ok(found) if found.is_empty() => println!("{label}: none"),
        Ok(found) => {
            for f in &found {
                println!("{label}: {f}");
            }
        }
        Err(e) => eprintln!("{label}: enumerate error: {e}"),
    }
}

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: coral-flash <firmware.bin>");
        std::process::exit(2);
    };
    let firmware = match std::fs::read(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("read firmware {path}: {e}");
            std::process::exit(2);
        }
    };

    report("before");
    if let Err(e) = cay::dfu::flash(&firmware) {
        eprintln!("flash failed: {e}");
        std::process::exit(1);
    }
    println!(
        "downloaded {} bytes; waiting for re-enumeration",
        firmware.len()
    );
    std::thread::sleep(Duration::from_secs(4));
    report("after");
}
