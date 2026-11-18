#[test]
fn test_disks() {
    let disks = sysinfo::Disks::new_with_refreshed_list();
    assert!(!disks.list().is_empty());
}
