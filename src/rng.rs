use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn range_inclusive(min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos() as u64);
    let mixed = time
        .wrapping_add(COUNTER.fetch_add(1, Ordering::Relaxed))
        .wrapping_mul(6364136223846793005)
        .rotate_left(17);
    min + mixed as u32 % (max - min + 1)
}

pub fn around(base: u32) -> u32 {
    let spread = (base / 4).max(1);
    range_inclusive(base.saturating_sub(spread), base.saturating_add(spread))
}
