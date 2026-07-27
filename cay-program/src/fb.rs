//! Minimal flatbuffer reads for fields planus cannot hand back as raw bytes.
//! MultiExecutable.serialized_executables is schema type `[string]` but holds
//! binary Executable buffers; planus's UTF-8-checked `&str` accessor rejects
//! them, so the vector is walked here and its elements returned as `&[u8]`.

use crate::error::{Error, Result};

fn off(a: usize, b: usize) -> Result<usize> {
    a.checked_add(b).ok_or(Error::Malformed("offset overflow"))
}

fn slice_at(buf: &[u8], pos: usize, len: usize) -> Result<&[u8]> {
    buf.get(pos..off(pos, len)?)
        .ok_or(Error::Malformed("read past end"))
}

fn u16_at(buf: &[u8], pos: usize) -> Result<u16> {
    let b = slice_at(buf, pos, 2)?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}

fn u32_at(buf: &[u8], pos: usize) -> Result<u32> {
    let b = slice_at(buf, pos, 4)?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn i32_at(buf: &[u8], pos: usize) -> Result<i32> {
    Ok(u32_at(buf, pos)? as i32)
}

/// The byte slices held by a MultiExecutable's `serialized_executables` vector
/// (field 0), each a serialized Executable. Every read is bounds-checked, so
/// any malformed or truncated input returns `Err`, never a panic.
pub fn executable_blobs(buf: &[u8]) -> Result<Vec<&[u8]>> {
    let root = u32_at(buf, 0)? as usize;
    let soffset = i32_at(buf, root)?;
    let vtable = usize::try_from(root as i64 - soffset as i64)
        .map_err(|_| Error::Malformed("bad vtable position"))?;

    let vtable_len = u16_at(buf, vtable)? as usize;
    if vtable_len < 6 {
        return Ok(Vec::new());
    }
    let field0 = u16_at(buf, off(vtable, 4)?)? as usize;
    if field0 == 0 {
        return Ok(Vec::new());
    }

    let field_pos = off(root, field0)?;
    let vec_pos = off(field_pos, u32_at(buf, field_pos)? as usize)?;
    let count = u32_at(buf, vec_pos)? as usize;

    let body_start = off(vec_pos, 4)?;
    let body_len = count
        .checked_mul(4)
        .ok_or(Error::Malformed("vector too large"))?;
    slice_at(buf, body_start, body_len)?;

    let mut out = Vec::with_capacity(count);
    for i in 0..count {
        let elem = off(body_start, i * 4)?;
        let s = off(elem, u32_at(buf, elem)? as usize)?;
        let len = u32_at(buf, s)? as usize;
        out.push(slice_at(buf, off(s, 4)?, len)?);
    }
    Ok(out)
}
