use metrics::{counter, gauge};

pub fn increment_indexed_blocks() {
    counter!("indexed_blocks_total", 1);
}

pub fn set_current_slot(slot: u64) {
    gauge!("current_slot", slot as f64);
}
