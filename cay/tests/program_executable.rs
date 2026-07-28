use cay::program::schema::{Executable, ExecutableType, OutputLayout, OutputLayoutRef};
use planus::{Builder, ReadAsRoot};

#[test]
fn output_layout_golden() {
    // Lifted from executable.fbs: the 2x2-tile → 4x5x32 output worked example.
    let mut b = Builder::new();
    let ol = OutputLayout::builder()
        .y_coordinate_to_linear_tile_id_map(vec![0, 0, 2, 2])
        .x_coordinate_to_linear_tile_id_map(vec![0, 0, 0, 1, 1])
        .linearized_tile_byte_offset(vec![0, 192, 320, 512])
        .x_coordinate_to_local_byte_offset(vec![0, 32, 64, 0, 32])
        .y_coordinate_to_local_y_offset(vec![0, 1, 0, 1])
        .x_coordinate_to_local_y_row_size(vec![96, 96, 96, 64, 64])
        .finish(&mut b);
    let buf = b.finish(ol, None).to_vec();

    let got: OutputLayout = OutputLayoutRef::read_as_root(&buf)
        .unwrap()
        .try_into()
        .unwrap();
    assert_eq!(
        got.y_coordinate_to_linear_tile_id_map,
        Some(vec![0, 0, 2, 2])
    );
    assert_eq!(
        got.x_coordinate_to_linear_tile_id_map,
        Some(vec![0, 0, 0, 1, 1])
    );
    assert_eq!(
        got.linearized_tile_byte_offset,
        Some(vec![0, 192, 320, 512])
    );
    assert_eq!(
        got.x_coordinate_to_local_byte_offset,
        Some(vec![0, 32, 64, 0, 32])
    );
    assert_eq!(got.y_coordinate_to_local_y_offset, Some(vec![0, 1, 0, 1]));
    assert_eq!(
        got.x_coordinate_to_local_y_row_size,
        Some(vec![96, 96, 96, 64, 64])
    );
}

#[test]
fn executable_fields_roundtrip() {
    let params = vec![1u8, 2, 3, 4, 5];
    let mut b = Builder::new();
    let ex = Executable::builder()
        .version_as_default()
        .name("yolo")
        .serialized_model_as_null()
        .batch_size(1)
        .scratch_size_bytes(4096)
        .instruction_bitstreams_as_null()
        .parameters(params.as_slice())
        .dma_hints_as_null()
        .input_layers_as_null()
        .output_layers_as_null()
        .chip("edgetpu")
        .estimated_cycles_as_default()
        .used_narrow_memory_bytes_per_tile(8192)
        .type_(ExecutableType::ParameterCaching)
        .parameter_caching_token(42)
        .use_tpu_dram_for_parameters_as_default()
        .estimated_cycles_64bit_as_default()
        .finish(&mut b);
    let buf = b.finish(ex, None).to_vec();

    let got = cay::program::parse_executable(&buf).unwrap();
    assert_eq!(got.name().unwrap(), Some("yolo"));
    assert_eq!(got.scratch_size_bytes().unwrap(), 4096);
    assert_eq!(got.parameters().unwrap(), Some(params.as_slice()));
    assert_eq!(got.used_narrow_memory_bytes_per_tile().unwrap(), 8192);
    assert_eq!(got.type_().unwrap(), ExecutableType::ParameterCaching);
    assert_eq!(got.parameter_caching_token().unwrap(), 42);
}
