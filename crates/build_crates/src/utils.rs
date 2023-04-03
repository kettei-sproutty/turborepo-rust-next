use chrono::Local;
use colored::Colorize;
use crossbeam_channel as channel;
use std::io::BufRead;

pub fn log_output<T: BufRead>(reader: T, prefix: &str, tx: channel::Sender<String>) {
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
                let formatted_line = format!(
                    "{} [BUILD TOOLS - {}] {}",
                    timestamp.dimmed(),
                    prefix.blue(),
                    line
                );
                tx.send(formatted_line).unwrap();
            }
            Err(error) => {
                let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
                let error_line = format!(
                    "{} [BUILD TOOLS - {}] Error: {}",
                    timestamp.dimmed(),
                    prefix.blue(),
                    error
                );
                tx.send(error_line).unwrap();
            }
        }
    }
}

pub fn log_error(message: &str) {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    println!("{} {}", timestamp.dimmed(), message.red());
}

pub fn log_info(message: &str) {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();
    println!("{} {}", timestamp.dimmed(), message.blue());
}

pub fn is_cargo_workspace() -> bool {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").unwrap();
    cargo_toml.contains("[workspace]")
}
