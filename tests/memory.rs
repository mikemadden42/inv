#[test]
fn test_memory() {
    let s = sysinfo::System::new_all();
    assert!(s.total_memory() > 0);
}
