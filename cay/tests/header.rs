use cay::ml::{packet_header, DescriptorTag};

#[test]
fn header_encodes_length_le_then_tag() {
    let h = packet_header(DescriptorTag::Parameters, 0x0011_2233);
    assert_eq!(&h[..4], &[0x33, 0x22, 0x11, 0x00]); // length, little-endian
    assert_eq!(h[4], 2); // Parameters tag in the low nibble
    assert_eq!(&h[5..], &[0, 0, 0]); // padding
}

#[test]
fn tag_occupies_low_nibble() {
    assert_eq!(packet_header(DescriptorTag::Instructions, 0)[4], 0);
    assert_eq!(packet_header(DescriptorTag::InputActivations, 1)[4], 1);
    assert_eq!(packet_header(DescriptorTag::OutputActivations, 999)[4], 3);
}
