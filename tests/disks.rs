#[test]
fn test_disks() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(!s.disks().is_empty());
}
