//! DMA chunking for the bulk streams. Port of libedgetpu's DmaChunker: hands out
//! successive chunks of a stream and tracks progress. In committed mode a chunk
//! is always fully processed, so the next chunk follows it; in best-effort mode a
//! chunk may be partially transferred and the unfinished tail is re-offered.

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Processing {
    Committed,
    BestEffort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chunk {
    pub offset: usize,
    pub len: usize,
}

pub struct DmaChunker {
    processing: Processing,
    size: usize,
    active: usize,
    transferred: usize,
}

impl DmaChunker {
    pub fn new(processing: Processing, size: usize) -> Self {
        Self {
            processing,
            size,
            active: 0,
            transferred: 0,
        }
    }

    fn next_offset(&self) -> usize {
        match self.processing {
            Processing::Committed => self.transferred + self.active,
            Processing::BestEffort => self.transferred,
        }
    }

    pub fn has_next_chunk(&self) -> bool {
        self.next_offset() < self.size
    }

    fn mark_active(&mut self, len: usize) {
        match self.processing {
            Processing::Committed => self.active += len,
            Processing::BestEffort => self.active = len,
        }
    }

    /// The next chunk, covering all remaining bytes.
    pub fn next_chunk(&mut self) -> Chunk {
        let offset = self.next_offset();
        let len = self.size.saturating_sub(offset);
        self.mark_active(len);
        Chunk { offset, len }
    }

    /// The next chunk, capped at `max_bytes`.
    pub fn next_chunk_upto(&mut self, max_bytes: usize) -> Chunk {
        let offset = self.next_offset();
        let len = self.size.saturating_sub(offset).min(max_bytes);
        self.mark_active(len);
        Chunk { offset, len }
    }

    /// Records that `bytes` of the active chunk were transferred.
    pub fn notify_transfer(&mut self, bytes: usize) -> Result<()> {
        if bytes > self.active || self.transferred + bytes > self.size {
            return Err(Error::InvalidTransfer);
        }
        self.transferred += bytes;
        match self.processing {
            Processing::Committed => self.active -= bytes,
            Processing::BestEffort => self.active = 0,
        }
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.active > 0
    }

    pub fn is_completed(&self) -> bool {
        self.transferred == self.size
    }

    /// Number of outstanding transfers of `bytes` each, rounded up.
    pub fn active_counts(&self, bytes: usize) -> usize {
        if bytes == 0 {
            return 0;
        }
        self.active.div_ceil(bytes)
    }
}
