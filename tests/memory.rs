#[test]
fn test_memory() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(s.total_memory() > 0);
}
