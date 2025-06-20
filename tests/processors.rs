use sysinfo::System;

#[test]
fn test_memory() {
    let s = sysinfo::System::new_all();
    assert!(!s.processes().is_empty());
    assert!(System::physical_core_count().unwrap() > 0);
    assert!(!s.cpus().is_empty());
}
