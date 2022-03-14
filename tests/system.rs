#[test]
fn test_sys_name() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(s.name().is_some());
}

#[test]
fn test_sys_kernel_version() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(s.kernel_version().is_some());
}

#[test]
fn test_sys_os_version() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(s.os_version().is_some());
}

#[test]
fn test_sys_host_name() {
    use sysinfo::SystemExt;
    let s = sysinfo::System::new_all();
    assert!(s.host_name().is_some());
}
