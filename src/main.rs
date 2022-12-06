use sysinfo::{System, SystemExt};
use tabled::{Rotate, Style, Table, Tabled};

fn main() {
    #[derive(Tabled)]
    struct SystemInfo {
        logical_cores: usize,
        physical_cores: usize,
        total_memory: u64,
        total_swap: u64,
        used_memory: u64,
        used_swap: u64,
        sys_name: String,
        sys_kernel_version: String,
        sys_os_version: String,
        sys_host_name: String,
    }

    let sys = System::new_all();

    let stats = vec![SystemInfo {
        logical_cores: sys.cpus().len(),
        physical_cores: sys.physical_core_count().unwrap(),
        total_memory: sys.total_memory(),
        total_swap: sys.total_swap(),
        used_memory: sys.used_memory(),
        used_swap: sys.used_swap(),
        sys_name: sys.name().unwrap(),
        sys_kernel_version: sys.kernel_version().unwrap(),
        sys_os_version: sys.os_version().unwrap(),
        sys_host_name: sys.host_name().unwrap(),
    }];

    let mut binding = Table::new(&stats);
    let table = binding.with(Rotate::Left).with(Style::modern());
    println!("{table}");

    println!("components:");
    for component in sys.components() {
        println!("{:?}", component);
    }
}
