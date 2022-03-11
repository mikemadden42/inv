use sysinfo::{NetworkExt, System, SystemExt};

fn main() {
    struct SystemInfo {
        logical_cores: usize,
        physical_cores: usize,
    }

    let sys = System::new_all();

    // We display the disks:
    println!("=> disk list:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    // Network data:
    for (interface_name, data) in sys.networks() {
        println!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }

    // Components temperature:
    for component in sys.components() {
        println!("{:?}", component);
    }

    // Memory information:
    println!("total memory: {} KB", sys.total_memory());
    println!("used memory : {} KB", sys.used_memory());
    println!("total swap  : {} KB", sys.total_swap());
    println!("used swap   : {} KB", sys.used_swap());

    let system_info = SystemInfo {
        logical_cores: sys.processors().len(),
        physical_cores: sys.physical_core_count().unwrap(),
    };

    println!("Number of cores: {}", system_info.logical_cores);
    println!(
        "Number of physical processors: {:?}",
        system_info.physical_cores
    );
}
