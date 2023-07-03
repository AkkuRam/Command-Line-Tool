use std::{alloc::System, env};
use sysinfo::{CpuExt, System as SysinfoSystem, SystemExt, ComponentExt};
fn main() {
    // command line arguments
    // let args: Vec<String> = env::args().collect();
    let mut sys = SysinfoSystem::new_all();
    println!("System info ");
    println!("total memory: {}", convert_bytes(sys.total_memory()));
    println!("used memory: {}", convert_bytes(sys.used_memory()));

    println!("NB CPUs: {}", sys.cpus().len());
}

fn convert_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
