use std::io::{self, Write};
use std::process::ExitCode;

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

fn run(out: &mut impl Write) -> io::Result<()> {
    let sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::nothing()),
    );

    writeln!(out, "system hostname: {}", or_unknown(System::host_name()))?;
    writeln!(out, "system type: {}", or_unknown(System::name()))?;
    writeln!(out, "system os: {}", or_unknown(System::os_version()))?;
    writeln!(
        out,
        "system kernel: {}",
        or_unknown(System::kernel_version())
    )?;

    // CPU information
    writeln!(out, "logical CPUs: {}", sys.cpus().len())?;
    writeln!(
        out,
        "physical cores: {}",
        System::physical_core_count().map_or_else(|| UNKNOWN.to_string(), |n| n.to_string())
    )?;

    // Memory information
    writeln!(out, "total memory: {}", format_bytes(sys.total_memory()))?;
    writeln!(out, "used memory : {}", format_bytes(sys.used_memory()))?;
    writeln!(out, "total swap  : {}", format_bytes(sys.total_swap()))?;
    writeln!(out, "used swap   : {}", format_bytes(sys.used_swap()))?;

    let components = Components::new_with_refreshed_list();
    if !components.is_empty() {
        writeln!(out, "temperatures:")?;
        for component in &components {
            writeln!(out, "  {component:?}")?;
        }
    }

    out.flush()
}

fn main() -> ExitCode {
    match run(&mut io::stdout().lock()) {
        Ok(()) => ExitCode::SUCCESS,
        // The reader went away (e.g. `inv | head`); that's not an error.
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("inv: {e}");
            ExitCode::FAILURE
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
    fn run_writes_report() {
        let mut out = Vec::new();
        run(&mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert!(text.starts_with("system hostname: "));
        assert!(text.contains("total memory: "));
    }

    #[test]
    fn or_unknown_falls_back() {
        assert_eq!(or_unknown(None), "unknown");
        assert_eq!(or_unknown(Some("x".into())), "x");
    }
}
