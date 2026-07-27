use cay_program::schema::{Description, InstructionBitstream, InstructionBitstreamRef, Position};
use cay_program::{copy_u32, field_sites, link_input, link_parameter, read_u32, FieldSite};
use planus::{Builder, ReadAsRoot};

#[test]
fn copy_read_u32_roundtrip() {
    let mut s: u64 = 0xc0ff_ee12_3456_789a;
    let mut next = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    for _ in 0..20_000 {
        let mut buf = vec![0u8; 12];
        let bit = (next() % 40) as usize;
        let val = next() as u32;
        copy_u32(&mut buf, bit, val).unwrap();
        assert_eq!(read_u32(&buf, bit).unwrap(), val);
    }
}

#[test]
fn copy_u32_byte_aligned() {
    let mut buf = [0u8; 8];
    copy_u32(&mut buf, 0, 0xDEAD_BEEF).unwrap();
    assert_eq!(buf, [0xEF, 0xBE, 0xAD, 0xDE, 0, 0, 0, 0]);
}

#[test]
fn copy_u32_byte_offset() {
    let mut buf = [0u8; 8];
    copy_u32(&mut buf, 8, 0x1122_3344).unwrap();
    assert_eq!(&buf[..5], [0x00, 0x44, 0x33, 0x22, 0x11]);
}

#[test]
fn copy_u32_bit_offset() {
    let mut buf = [0u8; 6];
    copy_u32(&mut buf, 4, 0x0000_ABCD).unwrap();
    assert_eq!(&buf[..5], [0xD0, 0xBC, 0x0A, 0x00, 0x00]);
}

#[test]
fn copy_u32_preserves_surrounding_bits() {
    let mut buf = [0xFFu8; 6];
    copy_u32(&mut buf, 4, 0).unwrap();
    assert_eq!(buf, [0x0F, 0x00, 0x00, 0x00, 0xF0, 0xFF]);
}

#[test]
fn copy_u32_out_of_bounds_errors() {
    let mut buf = [0u8; 3];
    assert!(copy_u32(&mut buf, 8, 1).is_err());
}

#[test]
fn copy_u32_never_panics() {
    let mut s: u64 = 0x243F_6A88_85A3_08D3;
    let mut next = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    for _ in 0..50_000 {
        let len = (next() % 12) as usize;
        let mut buf = vec![0u8; len];
        let off = (next() % 160) as usize;
        let _ = copy_u32(&mut buf, off, next() as u32);
    }
}

fn site(
    desc: Description,
    name: Option<&str>,
    batch: usize,
    position: Position,
    offset_bit: usize,
) -> FieldSite {
    FieldSite {
        desc,
        name: name.map(str::to_owned),
        batch,
        position,
        offset_bit,
    }
}

#[test]
fn link_parameter_writes_both_halves() {
    let sites = vec![
        site(
            Description::BaseAddressParameter,
            None,
            0,
            Position::Lower32bit,
            0,
        ),
        site(
            Description::BaseAddressParameter,
            None,
            0,
            Position::Upper32bit,
            32,
        ),
    ];
    let mut buf = [0u8; 8];
    link_parameter(&mut buf, &sites, 0x1122_3344_5566_7788).unwrap();
    assert_eq!(buf, [0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11]);
}

#[test]
fn link_input_matches_name_and_batch() {
    let sites = vec![site(
        Description::BaseAddressInputActivation,
        Some("images"),
        1,
        Position::Lower32bit,
        0,
    )];

    let mut buf = [0u8; 4];
    link_input(&mut buf, &sites, "images", &[0xAAAA, 0x0000_BBBB]).unwrap();
    assert_eq!(buf, [0xBB, 0xBB, 0x00, 0x00]);

    let mut other = [0u8; 4];
    link_input(&mut other, &sites, "boxes", &[0xAAAA, 0xBBBB]).unwrap();
    assert_eq!(other, [0u8; 4]);

    let mut short = [0u8; 4];
    assert!(link_input(&mut short, &sites, "images", &[0xAAAA]).is_err());
}

#[test]
fn field_sites_reads_planus_metadata() {
    let mut b = Builder::new();
    let meta = cay_program::schema::Meta::builder()
        .desc(Description::BaseAddressParameter)
        .batch(0)
        .name_as_null()
        .position(Position::Upper32bit)
        .finish(&mut b);
    let fo = cay_program::schema::FieldOffset::builder()
        .meta(meta)
        .offset_bit(96)
        .finish(&mut b);
    let ibs = InstructionBitstream::builder()
        .bitstream(vec![0u8; 16])
        .field_offsets(vec![fo])
        .finish(&mut b);
    let buf = b.finish(ibs, None).to_vec();

    let sites = field_sites(InstructionBitstreamRef::read_as_root(&buf).unwrap()).unwrap();
    assert_eq!(sites.len(), 1);
    assert_eq!(sites[0].desc, Description::BaseAddressParameter);
    assert_eq!(sites[0].position, Position::Upper32bit);
    assert_eq!(sites[0].offset_bit, 96);
    assert_eq!(sites[0].batch, 0);
    assert!(sites[0].name.is_none());
}
