#[test]
#[ignore]
fn test_components() {
    use sysinfo::Components;
    let components: Components = Components::new_with_refreshed_list();
    assert!(!components.is_empty());
}
