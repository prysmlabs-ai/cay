use cay::ml::{parse_event, parse_interrupt};

#[test]
fn parse_event_reads_offset_length_tag() {
    let mut b = [0u8; 16];
    b[0..8].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
    b[8..12].copy_from_slice(&0x0000_1000u32.to_le_bytes());
    b[12] = 0x13; // low nibble 3 = output-activations tag
    let e = parse_event(&b).unwrap();
    assert_eq!(e.offset, 0x1122_3344_5566_7788);
    assert_eq!(e.length, 0x1000);
    assert_eq!(e.tag, 3);
}

#[test]
fn parse_event_rejects_short() {
    assert!(parse_event(&[0u8; 8]).is_none());
    assert!(parse_event(&[]).is_none());
}

#[test]
fn parse_interrupt_reads_word() {
    assert_eq!(
        parse_interrupt(&0xdead_beefu32.to_le_bytes()),
        Some(0xdead_beef)
    );
    assert!(parse_interrupt(&[0u8; 2]).is_none());
}
