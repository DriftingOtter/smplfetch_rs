use chrono::Local;
use clap::Parser;
use serde::Serialize;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result as IoResult},
    path::Path,
};
use sysinfo::System;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use whoami::{arch, distro, username};

/// Simple system information CLI tool
#[derive(Parser)]
#[command(name = "smplfetch", version, about = "A Simple System Information Fetch Script.")]
struct Cli {
    /// Display color strip
    #[arg(short = 'c', long = "color-strip")]
    color_strip: bool,

    /// Skip battery information
    #[arg(short = 'b', long = "no-battery")]
    no_battery: bool,

    /// Output in JSON format
    #[arg(short = 'j', long = "json")]
    json: bool,

    /// Minimal info (time, user, memory)
    #[arg(short = 'm', long = "minimal")]
    minimal: bool,
}

#[derive(Serialize)]
struct SystemInfo {
    time: String,
    user: String,
    distro: Option<String>,
    arch: Option<String>,
    memory: String,
    battery: Option<String>,
}

const COLORS: &[Color] = &[
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
    Color::White,
];

fn get_current_time() -> String {
    Local::now().format("%I:%M %p").to_string()
}

fn get_memory_usage(sys: &mut System) -> String {
    sys.refresh_all();
    let total = sys.total_memory() as f64 / 1_073_741_824.0;
    let used  = sys.used_memory()  as f64 / 1_073_741_824.0;
    format!("{:.1}/{:.1} GB", used, total)
}

fn get_battery_percentage() -> String {
    ["/sys/class/power_supply/BAT0/capacity", "/sys/class/power_supply/BAT1/capacity"]
        .iter()
        .find_map(|p| read_battery_file(Path::new(p)).ok())
        .unwrap_or_else(|| "N/A".into())
}

fn read_battery_file(path: &Path) -> IoResult<String> {
    let file = File::open(path)?;
    let mut rdr = BufReader::new(file);
    let mut line = String::with_capacity(4);
    rdr.read_line(&mut line)?;
    Ok(line.trim().to_string())
}

fn generate_color_strip() {
    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
    let mut spec   = ColorSpec::new();
    for &c in COLORS {
        spec.set_fg(Some(c));
        if stdout.set_color(&spec).is_ok() {
            print!("██");
        }
    }
    let _ = stdout.reset();
    println!();
}

fn collect_system_info(skip_battery: bool, minimal: bool) -> SystemInfo {
    let mut sys = System::new();
    SystemInfo {
        time:    get_current_time(),
        user:    username(),
        distro:  (!minimal).then(|| distro()),
        arch:    (!minimal).then(|| arch().to_string()),
        memory:  get_memory_usage(&mut sys),
        battery: (!skip_battery).then(|| get_battery_percentage()),
    }
}

fn print_human_readable(info: &SystemInfo) {
    println!("{:<8} {}", "Time:",   info.time);
    println!("{:<8} {}", "User:",   info.user);
    if let Some(d) = &info.distro  { println!("{:<8} {}", "Dist:",   d); }
    if let Some(a) = &info.arch    { println!("{:<8} {}", "Krnl:",   a); }
    println!("{:<8} {}", "Memory:", info.memory);
    if let Some(b) = &info.battery { println!("{:<8} {}", "Battery:", b); }
}

fn main() {
    let cli = Cli::parse();
    let info = collect_system_info(cli.no_battery, cli.minimal);

    if cli.json {
        println!("{}", serde_json::to_string_pretty(&info).unwrap());
    } else {
        print_human_readable(&info);
    }

    if cli.color_strip {
        generate_color_strip();
    }
}

