//! This module implements utilities functions.

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// Returns the current timestamp in milliseconds from the Unix epoch.
pub fn get_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("System clock panic!").as_millis() as _
}
