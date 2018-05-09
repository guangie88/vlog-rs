#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_debug_implementations, missing_docs, warnings)]

//! # vlog
//!
//! Macros to do stdout / stderr logs based on verbosity level.
//!
//! Useful for CLI applications. The default verbosity level is 0, and the
//! supported max verbosity level is 3, which is equivalent to `-vvv` flags seen
//! in most Linux CLI applications.
//!
//! # Example
//! ```
//! #[macro_use]
//! extern crate vlog;
//!
//! use vlog::{get_verbosity_level, set_verbosity_level};
//!
//! fn main() {
//!     // default verbosity level is 0
//!     assert_eq!(0, get_verbosity_level());
//!
//!     v0!("v0 stdout prints");
//!     v1!("v1 stdout won't print");
//!     v2!("v2 stdout won't print");
//!     v3!("v3 stdout won't print");
//!
//!     // set custom verbosity level
//!     set_verbosity_level(1);
//!     assert_eq!(1, get_verbosity_level());
//!
//!     v0!("{} stdout prints", "v0");
//!     v1!("{} stdout prints", "v1");
//!     v2!("{} stdout won't print", "v2");
//!     v3!("{} stdout won't print", "v3");
//!
//!     // set custom max verbosity level
//!     set_verbosity_level(3);
//!     assert_eq!(3, get_verbosity_level());
//!
//!     v0!("{} stdout prints", "v0");
//!     v1!("{} stdout prints", "v1");
//!     v2!("{} stdout prints", "v2");
//!     v3!("{} stdout prints", "v3");
//!
//!     // stderr macros also available
//!     ve0!("{} stderr prints", "ve0");
//!     ve1!("{} stderr prints", "ve1");
//!     ve2!("{} stderr prints", "ve2");
//!     ve3!("{} stderr prints", "ve3");
//! }
//! ```

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

use std::sync::atomic::{AtomicUsize, Ordering};

/// Application verbosity level value.
static mut VERBOSITY_LEVEL: AtomicUsize = AtomicUsize::new(0);

/// Sets the application verbosity level atomically. This method is thread-safe.
///
/// # Arguments
/// * `verbosity_level` - Verbosity level value. While there is no check on the
/// upper bound, the expected max level value is 3.
///
/// # Example
/// ```
/// use vlog::set_verbosity_level;
///
/// // min verbosity level
/// set_verbosity_level(0);
///
/// // max verbosity level
/// set_verbosity_level(3);
/// ```
#[inline]
pub fn set_verbosity_level(verbosity_level: usize) {
    unsafe { VERBOSITY_LEVEL.store(verbosity_level, Ordering::Relaxed) }
}

/// Gets the application verbosity level atomically. This method is thread-safe.
///
/// # Example
/// ```
/// use vlog::{get_verbosity_level, set_verbosity_level};
///
/// // default verbosity level
/// assert_eq!(0, get_verbosity_level());
///
/// // custom verbosity level
/// set_verbosity_level(1);
/// assert_eq!(1, get_verbosity_level());
/// ```
#[inline]
pub fn get_verbosity_level() -> usize {
    unsafe { VERBOSITY_LEVEL.load(Ordering::Relaxed) }
}
