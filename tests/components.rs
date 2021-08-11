#[test]
#[ignore]
fn test_components() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(!s.components().is_empty());
}
