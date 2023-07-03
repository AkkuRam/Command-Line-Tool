use std::{alloc::System, env, io};
use sysinfo::{ComponentExt, CpuExt, System as Sysinfo, SystemExt};
use tui::{
    backend::{CrosstermBackend, Backend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal, Frame
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
fn main() -> Result<(), io::Error>  {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())

    // let mut sys = Sysinfo::new_all();
    // println!("System info ");
    // println!("total memory: {}", convert_bytes(sys.total_memory()));
    // println!("used memory: {}", convert_bytes(sys.used_memory()));
    // println!("NB CPUs: {}", sys.cpus().len());
    // cpu_usage(sys);
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

fn cpu_usage(mut sys: Sysinfo) {
    loop {
        sys.refresh_cpu();
        for cpu in sys.cpus() {
            print!("{}% ", cpu.cpu_usage());
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

