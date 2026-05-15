#[test]
fn version_constant_is_not_empty() {
    assert!(!bsort::VERSION.is_empty());
}

#[test]
fn run_succeeds_for_empty_scaffold() {
    assert!(bsort::run().is_ok());
}
