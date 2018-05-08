use super::*;

#[cfg(target_os = "linux")]
use gag::BufferRedirect;
#[cfg(target_os = "linux")]
use std::io::Read;
#[cfg(target_os = "linux")]
use std::thread::sleep;
#[cfg(target_os = "linux")]
use std::time::Duration;

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

#[cfg(target_os = "linux")]
#[test]
fn test_macros() {
    // This test has to be run without other tests being run since it depends on
    // the stdout/stderr.
    // While channel might be the better method, it is difficult to set up.
    // It is much easier to just sleep long enough to make sure this runs last.

    sleep(Duration::from_secs(1));

    set_verbosity_level(0);

    // stdout-0
    {
        let mut buf = BufferRedirect::stdout().unwrap();

        v0!("{}{}", "stdout", "0");
        v1!("{}{}", "stdout", "1");
        v2!("{}{}", "stdout", "2");
        v3!("{}{}", "stdout", "3");

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!("stdout0\n", &output);
    }

    // stderr-0
    {
        let mut buf = BufferRedirect::stderr().unwrap();

        ve0!("{}{}", "stderr", "0");
        ve1!("{}{}", "stderr", "1");
        ve2!("{}{}", "stderr", "2");
        ve3!("{}{}", "stderr", "3");

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!("stderr0\n", &output);
    }

    set_verbosity_level(1);

    // stdout-1
    {
        set_verbosity_level(1);
        let mut buf = BufferRedirect::stdout().unwrap();

        v0!("{}{}", "stdout", "0");
        v1!("{}{}", "stdout", "1");
        v2!("{}{}", "stdout", "2");
        v3!("{}{}", "stdout", "3");

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!("stdout0\nstdout1\n", &output);
    }

    // stderr-1
    {
        let mut buf = BufferRedirect::stderr().unwrap();

        ve0!("{}{}", "stderr", "0");
        ve1!("{}{}", "stderr", "1");
        ve2!("{}{}", "stderr", "2");
        ve3!("{}{}", "stderr", "3");

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!("stderr0\nstderr1\n", &output);
    }

    set_verbosity_level(2);

    // stdout-2
    {
        let mut buf = BufferRedirect::stdout().unwrap();

        v0!("{}{}", "stdout", "0");
        v1!("{}{}", "stdout", "1");
        v2!("{}{}", "stdout", "2");
        v3!("{}{}", "stdout", "3");

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!("stdout0\nstdout1\nstdout2\n", &output);
    }

    // stderr-2
    {
        let mut buf = BufferRedirect::stderr().unwrap();

        ve0!("{}{}", "stderr", "0");
        ve1!("{}{}", "stderr", "1");
        ve2!("{}{}", "stderr", "2");
        ve3!("{}{}", "stderr", "3");

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!("stderr0\nstderr1\nstderr2\n", &output);
    }

    set_verbosity_level(3);

    // stdout-3
    {
        let mut buf = BufferRedirect::stdout().unwrap();

        v0!("{}{}", "stdout", "0");
        v1!("{}{}", "stdout", "1");
        v2!("{}{}", "stdout", "2");
        v3!("{}{}", "stdout", "3");

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!("stdout0\nstdout1\nstdout2\nstdout3\n", &output);
    }

    // stderr-3
    {
        let mut buf = BufferRedirect::stderr().unwrap();

        ve0!("{}{}", "stderr", "0");
        ve1!("{}{}", "stderr", "1");
        ve2!("{}{}", "stderr", "2");
        ve3!("{}{}", "stderr", "3");

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        assert_eq!("stderr0\nstderr1\nstderr2\nstderr3\n", &output);
    }
}
