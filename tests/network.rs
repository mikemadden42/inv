#[test]
fn test_network() {
    use sysinfo::Networks;
    let n = Networks::new_with_refreshed_list();
    assert!(n.iter().count() > 0);
}
