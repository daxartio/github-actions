#[cfg(not(feature = "uuid"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(feature = "uuid"))]
pub(crate) fn random_id() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let addr = &now as *const u128 as usize;
    format!("{:x}-{:x}", now, addr)
}

#[cfg(not(feature = "uuid"))]
pub(crate) fn delimiter() -> String {
    format!("ghadelimiter_{}", random_id())
}

#[cfg(feature = "uuid")]
pub(crate) fn random_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(feature = "uuid")]
pub(crate) fn delimiter() -> String {
    format!("ghadelimiter_{}", random_id())
}
