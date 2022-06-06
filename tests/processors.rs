#[test]
fn test_memory() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(!s.processes().is_empty());
    assert!(s.physical_core_count().unwrap() > 0);
    assert!(!s.cpus().is_empty());
}
