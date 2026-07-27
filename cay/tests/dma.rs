use cay::dma::{Chunk, DmaChunker, Processing};

#[test]
fn committed_hands_out_before_completion_then_advances() {
    let mut c = DmaChunker::new(Processing::Committed, 100);
    assert!(c.has_next_chunk());
    assert_eq!(c.next_chunk_upto(30), Chunk { offset: 0, len: 30 });
    // a committed chunk is always processed, so the next follows it immediately.
    assert_eq!(
        c.next_chunk_upto(30),
        Chunk {
            offset: 30,
            len: 30
        }
    );
    c.notify_transfer(30).unwrap();
    assert_eq!(
        c.next_chunk_upto(30),
        Chunk {
            offset: 60,
            len: 30
        }
    );
}

#[test]
fn best_effort_reoffers_untransferred_tail() {
    let mut c = DmaChunker::new(Processing::BestEffort, 100);
    assert_eq!(c.next_chunk_upto(30), Chunk { offset: 0, len: 30 });
    c.notify_transfer(10).unwrap(); // hardware only took 10 of the 30
    assert_eq!(
        c.next_chunk_upto(30),
        Chunk {
            offset: 10,
            len: 30
        }
    );
}

#[test]
fn full_chunk_covers_remainder_and_completes() {
    let mut c = DmaChunker::new(Processing::Committed, 40);
    assert_eq!(c.next_chunk(), Chunk { offset: 0, len: 40 });
    c.notify_transfer(40).unwrap();
    assert!(c.is_completed());
    assert!(!c.has_next_chunk());
}

#[test]
fn rejects_over_transfer() {
    let mut c = DmaChunker::new(Processing::Committed, 40);
    c.next_chunk_upto(10);
    assert!(c.notify_transfer(20).is_err());
}

#[test]
fn active_counts_rounds_up() {
    let mut c = DmaChunker::new(Processing::Committed, 100);
    c.next_chunk_upto(50);
    assert_eq!(c.active_counts(16), 4); // ceil(50 / 16)
    assert_eq!(c.active_counts(0), 0);
}
