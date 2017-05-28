#[test]
fn empty_test() {}

#[test]
fn function_syntax_accepts_some_enum_as_some() {
    let input = Some(42u8);
    let expected_result = true;

    assert!(is_type!(input, Some(_)) == expected_result);
}

#[test]
fn function_syntax_rejects_none_enum_as_some() {
    #[allow(unused_qualifications)] //silences incorrect warning
    let input = Option::None::<u8>;
    let expected_result = false;

    assert!(is_type!(input, Some(_)) == expected_result);
}
