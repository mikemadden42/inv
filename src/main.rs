use sysinfo::{Components, CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

const UNKNOWN: &str = "unknown";

fn or_unknown(value: Option<String>) -> String {
    value.unwrap_or_else(|| UNKNOWN.to_string())
}

/// Formats a byte count as GiB with two decimal places.
fn format_bytes(bytes: u64) -> String {
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;
    format!("{:.2} GiB", bytes as f64 / GIB)
}

fn main() {
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::nothing()),
    );

    println!("system hostname: {}", or_unknown(System::host_name()));
    println!("system type: {}", or_unknown(System::name()));
    println!("system os: {}", or_unknown(System::os_version()));
    println!("system kernel: {}", or_unknown(System::kernel_version()));

    // CPU information
    println!("logical CPUs: {}", sys.cpus().len());
    println!(
        "physical cores: {}",
        System::physical_core_count().map_or_else(|| UNKNOWN.to_string(), |n| n.to_string())
    );

    // Memory information
    println!("total memory: {}", format_bytes(sys.total_memory()));
    println!("used memory : {}", format_bytes(sys.used_memory()));
    println!("total swap  : {}", format_bytes(sys.total_swap()));
    println!("used swap   : {}", format_bytes(sys.used_swap()));

    let components = Components::new_with_refreshed_list();
    if !components.is_empty() {
        println!("temperatures:");
        for component in &components {
            println!("  {component:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes_zero() {
        assert_eq!(format_bytes(0), "0.00 GiB");
    }

    #[test]
    fn format_bytes_whole_gib() {
        assert_eq!(format_bytes(24 * 1024 * 1024 * 1024), "24.00 GiB");
    }

    #[test]
    fn format_bytes_fractional() {
        assert_eq!(format_bytes(1024 * 1024 * 512), "0.50 GiB");
    }

    #[test]
    fn or_unknown_falls_back() {
        assert_eq!(or_unknown(None), "unknown");
        assert_eq!(or_unknown(Some("x".into())), "x");
    }
}
