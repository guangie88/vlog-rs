#![cfg_attr(feature = "cargo-clippy", deny(clippy))]
#![deny(missing_debug_implementations, missing_docs, warnings)]

//! # vlog
//!
//! Macros to do stdout / stderr logs based on verbosity level.
//!
//! Useful for CLI applications. The default verbosity level is 0, and the
//! supported max verbosity level is 3, which is equivalent to `-vvv` flags seen
//! in most Linux CLI applications.

// TODO: `gag` does not seem to work properly anymore
// #[cfg(test)]
// extern crate gag;

use std::sync::atomic::{AtomicUsize, Ordering};

/// Application verbosity level value.
static mut VERBOSITY_LEVEL: AtomicUsize = AtomicUsize::new(0);

/// Sets the application verbosity level atomically. This method is thread-safe.
pub fn set_verbosity_level(verbosity_level: usize) {
    unsafe { VERBOSITY_LEVEL.store(verbosity_level, Ordering::Relaxed) }
}

/// Gets the application verbosity level atomically. This method is thread-safe.
pub fn get_verbosity_level() -> usize {
    unsafe { VERBOSITY_LEVEL.load(Ordering::Relaxed) }
}

/// Common macro for verbose `stdout` logs.
/// This is meant to be used internally only.
#[macro_export]
macro_rules! verbose_log {
    ($v:expr, $fmt:expr) => {
        if $v <= ::vlog::get_verbosity_level() {
            println!($fmt);
        }
    };
    ($v:expr, $fmt:expr, $($arg:tt)*) => {
        if $v <= ::vlog::get_verbosity_level() {
            println!($fmt, $($arg)*);
        }
    };
}

/// Common macro for verbose `stderr` logs.
/// This is meant to be used internally only.
#[macro_export]
macro_rules! verbose_elog {
    ($v:expr, $fmt:expr) => {
        if $v <= ::vlog::get_verbosity_level() {
            eprintln!($fmt);
        }
    };
    ($v:expr, $fmt:expr, $($arg:tt)*) => {
        if $v <= ::vlog::get_verbosity_level() {
            eprintln!($fmt, $($arg)*);
        }
    };
}

/// Always prints to `stdout`. Follows
/// [`format!`](https://doc.rust-lang.org/std/macro.format.html) style.
#[macro_export]
macro_rules! v0 {
    ($fmt:expr) => { println!($fmt); };
    ($fmt:expr, $($arg:tt)*) => { println!($fmt, $($arg)*); };
}

/// Prints to `stdout` if verbosity level is >= 1. Follows
/// [`format!`](https://doc.rust-lang.org/std/macro.format.html) style.
#[macro_export]
macro_rules! v1 {
    ($fmt:expr) => { verbose_log!(1, $fmt); };
    ($fmt:expr, $($arg:tt)*) => { verbose_log!(1, $fmt, $($arg)*); };
}

/// Prints to `stdout` if verbosity level is >= 2. Follows
/// [`format!`](https://doc.rust-lang.org/std/macro.format.html) style.
#[macro_export]
macro_rules! v2 {
    ($fmt:expr) => { verbose_log!(2, $fmt); };
    ($fmt:expr, $($arg:tt)*) => { verbose_log!(2, $fmt, $($arg)*); };
}

/// Prints to `stdout` if verbosity level is >= 3. Follows
/// [`format!`](https://doc.rust-lang.org/std/macro.format.html) style.
#[macro_export]
macro_rules! v3 {
    ($fmt:expr) => { verbose_log!(3, $fmt); };
    ($fmt:expr, $($arg:tt)*) => { verbose_log!(3, $fmt, $($arg)*); };
}

/// Always prints to `stderr`. Follows
/// [`format!`](https://doc.rust-lang.org/std/macro.format.html) style.
#[macro_export]
macro_rules! ve0 {
    ($fmt:expr) => { eprintln!($fmt); };
    ($fmt:expr, $($arg:tt)*) => { eprintln!($fmt, $($arg)*); };
}

/// Prints to `stderr` if verbosity level is >= 1. Follows
/// [`format!`](https://doc.rust-lang.org/std/macro.format.html) style.
#[macro_export]
macro_rules! ve1 {
    ($fmt:expr) => { verbose_elog!(1, $fmt); };
    ($fmt:expr, $($arg:tt)*) => { verbose_elog!(1, $fmt, $($arg)*); };
}

/// Prints to `stderr` if verbosity level is >= 2. Follows
/// [`format!`](https://doc.rust-lang.org/std/macro.format.html) style.
#[macro_export]
macro_rules! ve2 {
    ($fmt:expr) => { verbose_elog!(2, $fmt); };
    ($fmt:expr, $($arg:tt)*) => { verbose_elog!(2, $fmt, $($arg)*); };
}

/// Prints to `stderr` if verbosity level is >= 3. Follows
/// [`format!`](https://doc.rust-lang.org/std/macro.format.html) style.
#[macro_export]
macro_rules! ve3 {
    ($fmt:expr) => { verbose_elog!(3, $fmt); };
    ($fmt:expr, $($arg:tt)*) => { verbose_elog!(3, $fmt, $($arg)*); };
}

#[cfg(test)]
mod tests {
    use super::*;
    // use gag::BufferRedirect;
    // use std::io::Read;

    #[test]
    fn test_set_get() {
        set_verbosity_level(0);
        assert_eq!(0, get_verbosity_level());

        set_verbosity_level(1);
        assert_eq!(1, get_verbosity_level());

        set_verbosity_level(2);
        assert_eq!(2, get_verbosity_level());

        set_verbosity_level(3);
        assert_eq!(3, get_verbosity_level());
    }

    // #[test]
    // fn test_v0() {
    //     set_verbosity_level(0);

    //     let mut buf = BufferRedirect::stdout().unwrap();

    //     v0!("{} {}", "level", "0");
    //     v1!("{} {}", "level", "1");
    //     v2!("{} {}", "level", "2");
    //     v3!("{} {}", "level", "3");

    //     let mut output = String::new();
    //     buf.read_to_string(&mut output).unwrap();

    //     assert_eq!("level 0", &output);
    // }

    // #[test]
    // fn test_ve0() {
    //     set_verbosity_level(0);

    //     let mut buf = BufferRedirect::stderr().unwrap();

    //     ve0!("{} {}", "level", "0");
    //     ve1!("{} {}", "level", "1");
    //     ve2!("{} {}", "level", "2");
    //     ve3!("{} {}", "level", "3");

    //     let mut output = String::new();
    //     buf.read_to_string(&mut output).unwrap();

    //     assert_eq!("level 0", &output);
    // }
}
