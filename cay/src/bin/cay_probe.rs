fn main() {
    match cay::usb::find_coral() {
        Ok(found) if found.is_empty() => {
            eprintln!("no Coral USB device found");
            std::process::exit(1);
        }
        Ok(found) => {
            for f in &found {
                println!("{f}");
            }
        }
        Err(e) => {
            eprintln!("usb error: {e}");
            std::process::exit(2);
        }
    }
}
