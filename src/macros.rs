/// Common macro for verbose `stdout` logs.
/// This is meant to be used internally only.
#[macro_export]
macro_rules! verbose_log {
    ($v:expr, $fmt:expr) => {
        if $v <= $crate::get_verbosity_level() {
            println!($fmt);
        }
    };
    ($v:expr, $fmt:expr, $($arg:tt)*) => {
        if $v <= $crate::get_verbosity_level() {
            println!($fmt, $($arg)*);
        }
    };
}

/// Common macro for verbose `stderr` logs.
/// This is meant to be used internally only.
#[macro_export]
macro_rules! verbose_elog {
    ($v:expr, $fmt:expr) => {
        if $v <= $crate::get_verbosity_level() {
            eprintln!($fmt);
        }
    };
    ($v:expr, $fmt:expr, $($arg:tt)*) => {
        if $v <= $crate::get_verbosity_level() {
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
