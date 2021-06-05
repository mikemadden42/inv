#[test]
fn test_memory() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(!s.get_processes().is_empty());
    assert!(s.get_physical_core_count().unwrap() > 0);
}
