#[macro_use]
extern crate vlog;

use vlog::set_verbosity_level;

fn print_v0() {
    set_verbosity_level(0);
    println!("\nSet verbosity level to 0!");

    v0!("[STDOUT] v0");
    v0!("[STDOUT] {}", "formatted v0");
    v1!("[STDOUT] v1 - MUST NOT appear");
    v1!("[STDOUT] {}", "formatted v1 - MUST NOT appear");
    v2!("[STDOUT] v2 - MUST NOT appear");
    v2!("[STDOUT] {}", "formatted v2 - MUST NOT appear");
    v3!("[STDOUT] v3 - MUST NOT appear");
    v3!("[STDOUT] {}", "formatted v3 - MUST NOT appear");

    ve0!("[STDERR] v0");
    ve0!("[STDERR] {}", "formatted v0");
    ve1!("[STDERR] v1 - MUST NOT appear");
    ve1!("[STDERR] {}", "formatted v1 - MUST NOT appear");
    ve2!("[STDERR] v2 - MUST NOT appear");
    ve2!("[STDERR] {}", "formatted v2 - MUST NOT appear");
    ve3!("[STDERR] v3 - MUST NOT appear");
    ve3!("[STDERR] {}", "formatted v3 - MUST NOT appear");
}

fn print_v1() {
    set_verbosity_level(1);
    println!("\nSet verbosity level to 1!");

    v0!("[STDOUT] v0");
    v0!("[STDOUT] {}", "formatted v0");
    v1!("[STDOUT] v1");
    v1!("[STDOUT] {}", "formatted v1");
    v2!("[STDOUT] v2 - MUST NOT appear");
    v2!("[STDOUT] {}", "formatted v2 - MUST NOT appear");
    v3!("[STDOUT] v3 - MUST NOT appear");
    v3!("[STDOUT] {}", "formatted v3 - MUST NOT appear");

    ve0!("[STDERR] v0");
    ve0!("[STDERR] {}", "formatted v0");
    ve1!("[STDERR] v1");
    ve1!("[STDERR] {}", "formatted v1");
    ve2!("[STDERR] v2 - MUST NOT appear");
    ve2!("[STDERR] {}", "formatted v2 - MUST NOT appear");
    ve3!("[STDERR] v3 - MUST NOT appear");
    ve3!("[STDERR] {}", "formatted v3 - MUST NOT appear");
}

fn print_v2() {
    set_verbosity_level(2);
    println!("\nSet verbosity level to 2!");

    v0!("[STDOUT] v0");
    v0!("[STDOUT] {}", "formatted v0");
    v1!("[STDOUT] v1");
    v1!("[STDOUT] {}", "formatted v1");
    v2!("[STDOUT] v2");
    v2!("[STDOUT] {}", "formatted v2");
    v3!("[STDOUT] v3 - MUST NOT appear");
    v3!("[STDOUT] {}", "formatted v3 - MUST NOT appear");

    ve0!("[STDERR] v0");
    ve0!("[STDERR] {}", "formatted v0");
    ve1!("[STDERR] v1");
    ve1!("[STDERR] {}", "formatted v1");
    ve2!("[STDERR] v2");
    ve2!("[STDERR] {}", "formatted v2");
    ve3!("[STDERR] v3 - MUST NOT appear");
    ve3!("[STDERR] {}", "formatted v3 - MUST NOT appear");
}

fn print_v3() {
    set_verbosity_level(3);
    println!("\nSet verbosity level to 3!");

    v0!("[STDOUT] v0");
    v0!("[STDOUT] {}", "formatted v0");
    v1!("[STDOUT] v1");
    v1!("[STDOUT] {}", "formatted v1");
    v2!("[STDOUT] v2");
    v2!("[STDOUT] {}", "formatted v2");
    v3!("[STDOUT] v3");
    v3!("[STDOUT] {}", "formatted v3");

    ve0!("[STDERR] v0");
    ve0!("[STDERR] {}", "formatted v0");
    ve1!("[STDERR] v1");
    ve1!("[STDERR] {}", "formatted v1");
    ve2!("[STDERR] v2");
    ve2!("[STDERR] {}", "formatted v2");
    ve3!("[STDERR] v3");
    ve3!("[STDERR] {}", "formatted v3");
}

mod nest {
    use super::*;

    pub fn print_another_v3() {
        set_verbosity_level(3);
        println!("\nAnother round of verbosity level 3!");

        v0!("[STDOUT] v0");
        v0!("[STDOUT] {}", "formatted v0");
        v1!("[STDOUT] v1");
        v1!("[STDOUT] {}", "formatted v1");
        v2!("[STDOUT] v2");
        v2!("[STDOUT] {}", "formatted v2");
        v3!("[STDOUT] v3");
        v3!("[STDOUT] {}", "formatted v3");

        ve0!("[STDERR] v0");
        ve0!("[STDERR] {}", "formatted v0");
        ve1!("[STDERR] v1");
        ve1!("[STDERR] {}", "formatted v1");
        ve2!("[STDERR] v2");
        ve2!("[STDERR] {}", "formatted v2");
        ve3!("[STDERR] v3");
        ve3!("[STDERR] {}", "formatted v3");
    }
}

fn main() {
    print_v0();
    print_v1();
    print_v2();
    print_v3();
    nest::print_another_v3();
}
