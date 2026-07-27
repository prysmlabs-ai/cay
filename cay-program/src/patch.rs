//! Address linking for DarwiNN instruction bitstreams. Port of libedgetpu's
//! ExecutableUtil: base addresses resolved at load time are written into the
//! immediate fields the compiler marked with FieldOffsets. Bounds checks replace
//! libedgetpu's fatal CHECKs, so bad metadata returns `Err` instead of aborting.

use crate::error::{Error, Result};
use crate::schema::{Description, InstructionBitstreamRef, Position};

/// One field to link: which base address, which 32-bit half, and where.
#[derive(Debug, Clone)]
pub struct FieldSite {
    pub desc: Description,
    pub name: Option<String>,
    pub batch: usize,
    pub position: Position,
    pub offset_bit: usize,
}

/// The link sites declared by one instruction bitstream.
pub fn field_sites(ibs: InstructionBitstreamRef<'_>) -> Result<Vec<FieldSite>> {
    let mut out = Vec::new();
    let Some(offsets) = ibs.field_offsets()? else {
        return Ok(out);
    };
    for fo in offsets {
        let fo = fo?;
        let meta = fo.meta()?.ok_or(Error::MissingField("field_offset.meta"))?;
        out.push(FieldSite {
            desc: meta.desc()?,
            name: meta.name()?.map(str::to_owned),
            batch: meta.batch()?.max(0) as usize,
            position: meta.position()?,
            offset_bit: fo.offset_bit()?.max(0) as usize,
        });
    }
    Ok(out)
}

fn word(address: u64, position: Position) -> u32 {
    match position {
        Position::Upper32bit => (address >> 32) as u32,
        _ => address as u32,
    }
}

/// Writes the low 32 bits of `value` into `buffer` starting at bit `offset_bit`,
/// least-significant bit first, preserving the surrounding bits.
pub fn copy_u32(buffer: &mut [u8], offset_bit: usize, value: u32) -> Result<()> {
    let mut next_bit = offset_bit;
    let mut remaining = 32usize;
    let mut val = value;
    while remaining > 0 {
        let bit = next_bit % 8;
        let to_set = (8 - bit).min(remaining);
        let byte = buffer
            .get_mut(next_bit / 8)
            .ok_or(Error::Malformed("patch offset past end"))?;
        let src_mask = ((1u16 << to_set) - 1) as u8;
        let dst_mask = src_mask << bit;
        *byte = (*byte & !dst_mask) | ((val as u8 & src_mask) << bit);
        val >>= to_set;
        remaining -= to_set;
        next_bit += to_set;
    }
    Ok(())
}

/// Reads a 32-bit value previously written by `copy_u32` at bit `offset_bit`.
pub fn read_u32(buffer: &[u8], offset_bit: usize) -> Result<u32> {
    let mut next_bit = offset_bit;
    let mut remaining = 32usize;
    let mut got = 0u32;
    let mut shift = 0u32;
    while remaining > 0 {
        let bit = next_bit % 8;
        let take = (8 - bit).min(remaining);
        let byte = *buffer
            .get(next_bit / 8)
            .ok_or(Error::Malformed("read past end"))?;
        let mask = ((1u16 << take) - 1) as u8;
        got |= u32::from((byte >> bit) & mask) << shift;
        shift += take as u32;
        remaining -= take;
        next_bit += take;
    }
    Ok(got)
}

fn link_batched(
    buffer: &mut [u8],
    sites: &[FieldSite],
    target: Description,
    name: &str,
    addresses: &[u64],
) -> Result<()> {
    for site in sites {
        if site.desc != target || site.name.as_deref() != Some(name) {
            continue;
        }
        let address = *addresses
            .get(site.batch)
            .ok_or(Error::Malformed("batch index out of range"))?;
        copy_u32(buffer, site.offset_bit, word(address, site.position))?;
    }
    Ok(())
}

fn link_single(
    buffer: &mut [u8],
    sites: &[FieldSite],
    target: Description,
    address: u64,
) -> Result<()> {
    for site in sites.iter().filter(|s| s.desc == target) {
        copy_u32(buffer, site.offset_bit, word(address, site.position))?;
    }
    Ok(())
}

pub fn link_scratch(buffer: &mut [u8], sites: &[FieldSite], address: u64) -> Result<()> {
    link_single(buffer, sites, Description::BaseAddressScratch, address)
}

pub fn link_parameter(buffer: &mut [u8], sites: &[FieldSite], address: u64) -> Result<()> {
    link_single(buffer, sites, Description::BaseAddressParameter, address)
}

pub fn link_input(
    buffer: &mut [u8],
    sites: &[FieldSite],
    name: &str,
    addresses: &[u64],
) -> Result<()> {
    link_batched(
        buffer,
        sites,
        Description::BaseAddressInputActivation,
        name,
        addresses,
    )
}

pub fn link_output(
    buffer: &mut [u8],
    sites: &[FieldSite],
    name: &str,
    addresses: &[u64],
) -> Result<()> {
    link_batched(
        buffer,
        sites,
        Description::BaseAddressOutputActivation,
        name,
        addresses,
    )
}
