use cay::dfu::parse_transfer_size;

#[test]
fn reads_transfer_size_from_functional_descriptor() {
    // Interface descriptor (len 9, type 4) then the DFU functional descriptor
    // (len 9, type 0x21) whose wTransferSize at bytes 5..7 is 0x0040 = 64.
    let extra = [
        0x09, 0x04, 0x00, 0x00, 0x00, 0xfe, 0x01, 0x02, 0x00, // interface
        0x09, 0x21, 0x0d, 0xff, 0x00, 0x40, 0x00, 0x1a, 0x01, // dfu functional
    ];
    assert_eq!(parse_transfer_size(&extra), Some(64));
}

#[test]
fn returns_none_without_functional_descriptor() {
    assert_eq!(
        parse_transfer_size(&[0x09, 0x04, 0, 0, 0, 0, 0, 0, 0]),
        None
    );
}

#[test]
fn tolerates_truncated_descriptors() {
    assert_eq!(parse_transfer_size(&[0x09, 0x21, 0x0d]), None);
    assert_eq!(parse_transfer_size(&[]), None);
    assert_eq!(parse_transfer_size(&[0x00, 0x21]), None);
}
