use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn random_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let addr = &now as *const u128 as usize;
    format!("{:x}-{:x}", now, addr)
}

pub(crate) fn delimiter() -> String {
    format!("ghadelimiter_{}", random_id())
}
