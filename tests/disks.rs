#[test]
fn test_disks() {
    let disks = sysinfo::Disks::new();
    assert!(disks.list().is_empty());
}
