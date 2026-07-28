//! Output relayout. The accelerator writes its output tiled across the systolic
//! array; the OutputLayout maps from executable.fbs turn a (y, x) coordinate into
//! the byte offset of that element block in the tiled buffer, which this
//! reassembles into a row-major (y, x, z) tensor.

use crate::program::error::{Error, Result};
use crate::program::schema::OutputLayout;

fn at(map: &Option<Vec<i32>>, i: usize) -> Result<i64> {
    map.as_ref()
        .and_then(|v| v.get(i))
        .map(|&x| i64::from(x))
        .ok_or(Error::Malformed("output layout map index out of range"))
}

fn tile_offset(layout: &OutputLayout, y: usize, x: usize) -> Result<usize> {
    let tile = usize::try_from(
        at(&layout.y_coordinate_to_linear_tile_id_map, y)?
            + at(&layout.x_coordinate_to_linear_tile_id_map, x)?,
    )
    .map_err(|_| Error::Malformed("negative tile id"))?;
    let base = at(&layout.linearized_tile_byte_offset, tile)?;
    let local_y = at(&layout.y_coordinate_to_local_y_offset, y)?;
    let row_size = at(&layout.x_coordinate_to_local_y_row_size, x)?;
    let local_x = at(&layout.x_coordinate_to_local_byte_offset, x)?;
    usize::try_from(base + local_y * row_size + local_x)
        .map_err(|_| Error::Malformed("negative byte offset"))
}

/// Converts the accelerator's biased-unsigned output to the signed
/// representation the layer declares, by flipping the MSB of each scalar. Port
/// of libedgetpu's `TransformSignedDataType`: the MSB is the last byte of each
/// `scalar_bytes`-wide element (little-endian). Applied only to signed layers.
pub fn transform_signed(buf: &mut [u8], scalar_bytes: usize) {
    if scalar_bytes == 0 {
        return;
    }
    let mut msb = scalar_bytes - 1;
    while msb < buf.len() {
        buf[msb] ^= 0x80;
        msb += scalar_bytes;
    }
}

/// Reassembles a tiled output buffer into a row-major (y, x, z) tensor.
/// `element_bytes` is z_dim * bytes_per_element.
pub fn relayout(layout: &OutputLayout, tiled: &[u8], element_bytes: usize) -> Result<Vec<u8>> {
    let y_dim = layout
        .y_coordinate_to_local_y_offset
        .as_ref()
        .map_or(0, Vec::len);
    let x_dim = layout
        .x_coordinate_to_local_byte_offset
        .as_ref()
        .map_or(0, Vec::len);

    let total = y_dim
        .checked_mul(x_dim)
        .and_then(|n| n.checked_mul(element_bytes))
        .ok_or(Error::Malformed("output too large"))?;
    let mut out = vec![0u8; total];

    for y in 0..y_dim {
        for x in 0..x_dim {
            let src = tile_offset(layout, y, x)?;
            let end = src
                .checked_add(element_bytes)
                .ok_or(Error::Malformed("offset overflow"))?;
            let block = tiled
                .get(src..end)
                .ok_or(Error::Malformed("tiled buffer too short"))?;
            let dst = (y * x_dim + x) * element_bytes;
            out[dst..dst + element_bytes].copy_from_slice(block);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::transform_signed;

    #[test]
    fn signed_8bit_flips_every_byte() {
        let mut buf = vec![0x00, 0x03, 0x80, 0xff];
        transform_signed(&mut buf, 1);
        assert_eq!(buf, vec![0x80, 0x83, 0x00, 0x7f]);
    }

    #[test]
    fn signed_16bit_flips_only_high_byte_of_each_scalar() {
        // Little-endian: the MSB is the second byte of each 2-byte scalar.
        let mut buf = vec![0x11, 0x00, 0x22, 0x80];
        transform_signed(&mut buf, 2);
        assert_eq!(buf, vec![0x11, 0x80, 0x22, 0x00]);
    }

    #[test]
    fn zero_scalar_bytes_is_a_noop() {
        let mut buf = vec![1, 2, 3];
        transform_signed(&mut buf, 0);
        assert_eq!(buf, vec![1, 2, 3]);
    }
}
