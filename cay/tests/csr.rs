use cay::ml::{read_transfer, write_transfer};

#[test]
fn read32_splits_offset_and_is_vendor_in() {
    let t = read_transfer(0x1234_5678, 4);
    assert_eq!(t.request_type, 0xc0); // In | Vendor | Device
    assert_eq!(t.request, 1); // 32-bit id
    assert_eq!(t.value, 0x5678); // low 16 of offset
    assert_eq!(t.index, 0x1234); // high 16 of offset
    assert_eq!(t.length, 4);
}

#[test]
fn read64_uses_request_zero() {
    let t = read_transfer(0x0004_86c0, 8);
    assert_eq!(t.request, 0); // 64-bit id
    assert_eq!(t.length, 8);
    assert_eq!(t.value, 0x86c0);
    assert_eq!(t.index, 0x0004);
}

#[test]
fn write32_is_vendor_out() {
    let t = write_transfer(0xdead_beef, 4);
    assert_eq!(t.request_type, 0x40); // Out | Vendor | Device
    assert_eq!(t.request, 1);
    assert_eq!(t.value, 0xbeef);
    assert_eq!(t.index, 0xdead);
}
