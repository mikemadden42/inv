use sysinfo::{NetworkExt, System, SystemExt};

fn main() {
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

    // Number of processors
    println!("Number of cores: {}", sys.processors().len());
    println!(
        "Number of physical processors: {:?}",
        sys.physical_core_count().unwrap()
    );
}
