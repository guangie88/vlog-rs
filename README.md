# vlog-rs

[![Build Status](https://travis-ci.org/guangie88/vlog-rs.svg?branch=master)](https://travis-ci.org/guangie88/vlog-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/65p683ppij588sv8/branch/master?svg=true)](https://ci.appveyor.com/project/guangie88/vlog-rs/branch/master)
[![codecov](https://codecov.io/gh/guangie88/vlog-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/guangie88/vlog-rs)
[![Crates.io](https://img.shields.io/crates/v/vlog.svg)](https://crates.io/crates/vlog)
[![Docs.rs](https://docs.rs/vlog/badge.svg)](https://docs.rs/vlog)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Macros to do stdout / stderr logs based on verbosity level, which take in the
same arguments as
[`println!`](https://doc.rust-lang.org/1.0.0/std/macro.println!.html) macro.

Useful for CLI applications. The default verbosity level is 0, and the supported
max verbosity level is 3, which is equivalent to `-vvv` flags as seen in most
Linux CLI applications.

## Example

```rust
#[macro_use]
extern crate vlog;

use vlog::{get_verbosity_level, set_verbosity_level};

fn main() {
    // default verbosity level is 0
    assert_eq!(0, get_verbosity_level());

    v0!("v0 stdout prints");
    v1!("v1 stdout won't print");
    v2!("v2 stdout won't print");
    v3!("v3 stdout won't print");

    // set custom verbosity level
    set_verbosity_level(1);
    assert_eq!(1, get_verbosity_level());

    v0!("{} stdout prints", "v0");
    v1!("{} stdout prints", "v1");
    v2!("{} stdout won't print", "v2");
    v3!("{} stdout won't print", "v3");

    // set custom max verbosity level
    set_verbosity_level(3);
    assert_eq!(3, get_verbosity_level());

    v0!("{} stdout prints", "v0");
    v1!("{} stdout prints", "v1");
    v2!("{} stdout prints", "v2");
    v3!("{} stdout prints", "v3");

    // stderr macros also available
    ve0!("{} stderr prints", "ve0");
    ve1!("{} stderr prints", "ve1");
    ve2!("{} stderr prints", "ve2");
    ve3!("{} stderr prints", "ve3");
}
```
