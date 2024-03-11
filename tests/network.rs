#[test]
fn test_network() {
    use sysinfo::Networks;
    let n = Networks::new();
    assert_eq!(n.iter().count(), 0);
}
