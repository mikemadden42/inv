#[test]
fn test_network() {
    use sysinfo::{NetworksExt, SystemExt};
    let s = sysinfo::System::new();
    assert_eq!(s.networks().iter().count(), 0);
    let s = sysinfo::System::new_all();
    assert!(s.networks().iter().count() > 0);
}
