use chrono;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

fn get_username() -> String {
    return whoami::username();
}

fn get_distrobution_name() -> String {
    return whoami::distro();
}

fn get_cpu_architecture() -> whoami::Arch {
    return whoami::arch();
}

fn get_current_time() -> String {
    return chrono::Local::now().time().format("%H:%M").to_string();
}

fn get_memory_useage() -> String {
    let sys = sysinfo::System::new_all();

    let total_memory = sys.total_memory() as u64;
    let used_memory = sys.used_memory() as u64;

    let total_gb = total_memory / (1024 * 1024 * 1024);
    let used_gb = used_memory / (1024 * 1024 * 1024);

    let memory_usage = format!("{}/{} GB", used_gb, total_gb);

    return memory_usage;
}

fn get_battery_percentage() -> String {
    let paths = [
        Path::new("/sys/class/power_supply/BAT0/capacity"),
        Path::new("/sys/class/power_supply/BAT1/capacity"),
    ];

    for path in paths.iter() {
        if let Ok(file) = File::open(path) {
            let mut reader = BufReader::new(file);
            let mut line = String::new();

            match reader.read_line(&mut line) {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        line.pop();
                        return format!("{}%", line.parse::<u8>().unwrap_or(0));
                    }
                }
                Err(err) => eprintln!("Error reading file: {}", err),
            }
        }
    }
    "N/A".to_string()
}

fn generate_color_strip() {
    let colored_block = "██";

    let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);

    let colors = [
        (Color::Black, "Black"),
        (Color::Red, "Red"),
        (Color::Green, "Green"),
        (Color::Yellow, "Yellow"),
        (Color::Blue, "Blue"),
        (Color::Magenta, "Magenta"),
        (Color::Cyan, "Cyan"),
        (Color::White, "White"),
    ];

    let mut color_spec = ColorSpec::new();

    for (color, _color_name) in &colors {
        color_spec.set_fg(Some(*color));
        stdout.set_color(&color_spec).unwrap();

        print!("{}", colored_block);

        stdout.reset().unwrap();
    }
    println!("");
}

fn get_color_strip() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Invalid command-line argument. Please use the '-cs' flag to generate the color strip.");
        return;
    }

    if args.len() == 2 && (args[1] == "-cs" || args[1] == "--color-strip") {
        generate_color_strip();
    }
}

fn main() {
    println!("⎧                          ⎫");
    println!("⎮          ⎮{}⎮         ⎮", get_current_time());
    println!("⎩                          ⎭");

    println!("User: {}", get_username());
    println!("Dist: {}", get_distrobution_name());
    println!("Krnl: {}", get_cpu_architecture());

    println!("Memory: {}", get_memory_useage());
    println!("Battry: {}", get_battery_percentage());

    get_color_strip();
}

