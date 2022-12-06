use sysinfo::{System, SystemExt};

fn main() {
    let sys = System::new_all();

    println!("system hostname: {}", sys.host_name().unwrap());
    println!("system type: {}", sys.name().unwrap());
    println!("system os: {}", sys.os_version().unwrap());
    println!("system kernel: {}", sys.kernel_version().unwrap());

    // Number of processors
    println!("Number of cores: {}", sys.cpus().len());
    println!(
        "Number of physical processors: {:?}",
        sys.physical_core_count().unwrap()
    );

    // Memory information:
    println!("total memory: {} KB", sys.total_memory());
    println!("used memory : {} KB", sys.used_memory());
    println!("total swap  : {} KB", sys.total_swap());
    println!("used swap   : {} KB", sys.used_swap());

    // Components temperature:
    for component in sys.components() {
        println!("{:?}", component);
    }
}
