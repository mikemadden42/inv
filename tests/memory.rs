#[test]
fn test_memory() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(s.get_total_memory() > 0);
    assert!(s.get_total_swap() > 0);
}
