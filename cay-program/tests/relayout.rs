use cay_program::relayout;
use cay_program::schema::OutputLayout;

// The 2x2-tile → 4x5x32 worked example from executable.fbs.
fn worked_example() -> OutputLayout {
    OutputLayout {
        y_coordinate_to_linear_tile_id_map: Some(vec![0, 0, 2, 2]),
        x_coordinate_to_linear_tile_id_map: Some(vec![0, 0, 0, 1, 1]),
        linearized_tile_byte_offset: Some(vec![0, 192, 320, 512]),
        x_coordinate_to_local_byte_offset: Some(vec![0, 32, 64, 0, 32]),
        y_coordinate_to_local_y_offset: Some(vec![0, 1, 0, 1]),
        x_coordinate_to_local_y_row_size: Some(vec![96, 96, 96, 64, 64]),
    }
}

#[test]
fn relayout_matches_worked_example() {
    let layout = worked_example();
    let tiled: Vec<u8> = (0..640).map(|i| (i % 256) as u8).collect();
    let out = relayout(&layout, &tiled, 32).unwrap();

    // (y, x, tiled-source-byte), source offsets computed by hand from the maps.
    let cases = [
        (0usize, 0usize, 0usize),
        (0, 3, 192),
        (0, 4, 224),
        (1, 0, 96),
        (3, 0, 416),
        (3, 4, 608),
    ];
    for (y, x, src) in cases {
        let dst = (y * 5 + x) * 32;
        assert_eq!(
            &out[dst..dst + 32],
            &tiled[src..src + 32],
            "element ({y},{x})"
        );
    }
}

#[test]
fn relayout_rejects_short_buffer() {
    assert!(relayout(&worked_example(), &[0u8; 100], 32).is_err());
}
