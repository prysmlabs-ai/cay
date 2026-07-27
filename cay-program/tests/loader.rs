use cay_program::schema::{MultiExecutable, Package};
use planus::Builder;

fn build_multi(blobs: &[&str]) -> Vec<u8> {
    let mut b = Builder::new();
    let m = MultiExecutable::builder()
        .serialized_executables(blobs.to_vec())
        .finish(&mut b);
    b.finish(m, None).to_vec()
}

fn build_package(multi: &[u8]) -> Vec<u8> {
    let mut b = Builder::new();
    let pkg = Package::builder()
        .min_runtime_version(1)
        .serialized_multi_executable(multi)
        .signature_as_null()
        .keypair_version_as_default()
        .compiler_version("test-compiler")
        .virtual_chip_id_as_default()
        .multi_chip_package_as_null()
        .model_identifier_as_null()
        .finish(&mut b);
    // planus's finish-with-identifier writes the identifier first, which its own
    // reader can't consume; real DarwiNN files are standard flatc output with the
    // identifier at [4..8], so lay it out that way.
    add_darwinn_identifier(b.finish(pkg, None))
}

fn add_darwinn_identifier(buf: &[u8]) -> Vec<u8> {
    let root = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let mut out = Vec::with_capacity(buf.len() + 4);
    out.extend_from_slice(&(root + 4).to_le_bytes());
    out.extend_from_slice(b"DWN1");
    out.extend_from_slice(&buf[4..]);
    out
}

#[test]
fn blobs_match_planus_write_layout() {
    let blobs = ["executable-zero", "exec-one", "x"];
    let buf = build_multi(&blobs);
    let got = cay_program::executable_blobs(&buf).unwrap();
    assert_eq!(got.len(), blobs.len());
    for (slice, expected) in got.iter().zip(blobs.iter()) {
        assert_eq!(*slice, expected.as_bytes());
    }
}

#[test]
fn empty_multi_executable_yields_no_blobs() {
    let buf = build_multi(&[]);
    assert!(cay_program::executable_blobs(&buf).unwrap().is_empty());
}

#[test]
fn package_roundtrips_to_blobs() {
    let multi = build_multi(&["exec-a", "exec-b"]);
    let buf = build_package(&multi);

    let pkg = cay_program::parse_package(&buf).unwrap();
    let inner = cay_program::multi_executable_bytes(&pkg).unwrap();
    assert_eq!(inner, multi.as_slice());

    let got = cay_program::executable_blobs(inner).unwrap();
    assert_eq!(got, vec![b"exec-a".as_slice(), b"exec-b".as_slice()]);
}

#[test]
fn rejects_missing_identifier() {
    let buf = vec![0u8; 32];
    assert!(matches!(
        cay_program::parse_package(&buf),
        Err(cay_program::Error::BadIdentifier)
    ));
}

#[test]
fn parsers_never_panic_on_garbage() {
    let mut s: u64 = 0x9E37_79B9_7F4A_7C15;
    let mut next = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    for _ in 0..50_000 {
        let len = (next() % 96) as usize;
        let buf: Vec<u8> = (0..len).map(|_| next() as u8).collect();
        let _ = cay_program::executable_blobs(&buf);
        let _ = cay_program::parse_package(&buf);
    }
}

#[test]
fn truncations_never_panic() {
    let valid = build_package(&build_multi(&["exec-a", "exec-b", "exec-c"]));
    for cut in 0..valid.len() {
        let _ = cay_program::parse_package(&valid[..cut]);
        let _ = cay_program::executable_blobs(&valid[..cut]);
    }
}
