use sysinfo::{NetworkExt, System, SystemExt};

fn main() {
    let sys = System::new_all();

    // We display the disks:
    println!("=> disk list:");
    for disk in sys.get_disks() {
        println!("{:?}", disk);
    }

    // Network data:
    for (interface_name, data) in sys.get_networks() {
        println!(
            "{}: {}/{} B",
            interface_name,
            data.get_received(),
            data.get_transmitted()
        );
    }

    // Components temperature:
    for component in sys.get_components() {
        println!("{:?}", component);
    }

    // Memory information:
    println!("total memory: {} KB", sys.get_total_memory());
    println!("used memory : {} KB", sys.get_used_memory());
    println!("total swap  : {} KB", sys.get_total_swap());
    println!("used swap   : {} KB", sys.get_used_swap());

    // Number of processors
    println!("NB processors: {}", sys.get_processors().len());
}
