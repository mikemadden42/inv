use sysinfo::{Components, System};

fn main() {
    let sys = System::new_all();

    println!("system hostname: {}", System::host_name().unwrap());
    println!("system type: {}", System::name().unwrap());
    println!("system os: {}", System::os_version().unwrap());
    println!("system kernel: {}", System::kernel_version().unwrap());

    // Number of processors
    println!("Number of cores: {}", sys.cpus().len());
    println!(
        "Number of physical processors: {:?}",
        System::physical_core_count().unwrap() // Changed from sys.physical_core_count()
    );

    // Memory information:
    println!("total memory: {} KB", sys.total_memory());
    println!("used memory : {} KB", sys.used_memory());
    println!("total swap  : {} KB", sys.total_swap());
    println!("used swap   : {} KB", sys.used_swap());

    let components: Components = Components::new_with_refreshed_list();
    for component in &components {
        println!("{component:?}");
    }
}
