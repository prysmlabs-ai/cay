fn main() {
    let offset = std::env::args().nth(1).and_then(|s| {
        let hex = s.strip_prefix("0x").unwrap_or(&s);
        u32::from_str_radix(hex, 16).ok()
    });
    let Some(offset) = offset else {
        eprintln!("usage: coral-reg <hex-offset>");
        std::process::exit(2);
    };

    let csr = match cay::ml::Csr::open() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("open runtime device: {e}");
            std::process::exit(1);
        }
    };
    match csr.read32(offset) {
        Ok(v) => println!("CSR[0x{offset:x}] = 0x{v:08x}"),
        Err(e) => {
            eprintln!("read CSR 0x{offset:x}: {e}");
            std::process::exit(1);
        }
    }
}
