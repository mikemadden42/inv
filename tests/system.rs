use sysinfo::System;

#[test]
fn test_sys_name() {
    assert!(System::name().is_some());
}

#[test]
fn test_sys_kernel_version() {
    assert!(System::kernel_version().is_some());
}

#[test]
fn test_sys_os_version() {
    assert!(System::os_version().is_some());
}

#[test]
fn test_sys_host_name() {
    assert!(System::host_name().is_some());
}
