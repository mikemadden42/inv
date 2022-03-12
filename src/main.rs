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
    }

    let sys = System::new_all();

    let stats = vec![SystemInfo {
        logical_cores: sys.processors().len(),
        physical_cores: sys.physical_core_count().unwrap(),
        total_memory: sys.total_memory(),
        total_swap: sys.total_swap(),
        used_memory: sys.used_memory(),
        used_swap: sys.used_swap(),
    }];

    let table = Table::new(&stats).with(Rotate::Left).with(Style::modern());
    println!("{table}");
}
