use super::*;

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
