use crate::members::{Member, MemberTarget};
use crate::utils::{is_cargo_workspace, log_error, log_info, log_output};
use crossbeam_channel as channel;
use rayon::prelude::*;
use std::path::Path;
use std::process::exit;
use std::process::{Command, Stdio};
use std::time::Instant;

pub fn build_project(member: &Member, tx: channel::Sender<String>) {
    println!(
        "Building {} with thread ID: {:?}",
        member.path,
        std::thread::current().id()
    );

    match member.target {
        MemberTarget::Cargo => {
            println!("Building {} with Cargo", member.path);
            let manifest_path = Path::new(&member.path).join("Cargo.toml");

            let is_workspace = is_cargo_workspace();
            let command_args = if is_workspace {
                ["run", "-p", &member.path]
            } else {
                ["build", "--manifest-path", manifest_path.to_str().unwrap()]
            };

            let mut child = Command::new("cargo")
                .args(command_args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute Cargo build");

            if let Some(stdout) = child.stdout.take() {
                let tx_clone = tx.clone();
                let member_path_clone = member.path.clone();
                std::thread::spawn(move || {
                    log_output(
                        std::io::BufReader::new(stdout),
                        &member_path_clone,
                        tx_clone,
                    );
                });
            }

            if let Some(stderr) = child.stderr.take() {
                let tx_clone = tx;
                let member_path_clone = member.path.clone();
                std::thread::spawn(move || {
                    log_output(
                        std::io::BufReader::new(stderr),
                        &member_path_clone,
                        tx_clone,
                    );
                });
            }

            let status = child.wait().expect("Failed to wait on Cargo child process");

            if !status.success() {
                log_error(&format!(
                    "[BUILD TOOLS - {}] Error building with Cargo",
                    member.path
                ));

                if !member.is_skippable {
                    exit(-1);
                } else {
                    log_info(&format!(
                        "[BUILD TOOLS - {}] Skipping due to error, but continuing execution",
                        member.path
                    ));
                }
            }
        }
        MemberTarget::WasmPack => {
            println!("Building {} with wasm-pack", member.path);
            let mut child = Command::new("wasm-pack")
                .args(["build", &member.path, "--verbose"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to execute wasm-pack build");

            if let Some(stdout) = child.stdout.take() {
                let tx_clone = tx.clone();
                let member_path_clone = member.path.clone();
                std::thread::spawn(move || {
                    log_output(
                        std::io::BufReader::new(stdout),
                        &member_path_clone,
                        tx_clone,
                    );
                });
            }

            if let Some(stderr) = child.stderr.take() {
                let tx_clone = tx;
                let member_path_clone = member.path.clone();
                std::thread::spawn(move || {
                    log_output(
                        std::io::BufReader::new(stderr),
                        &member_path_clone,
                        tx_clone,
                    );
                });
            }

            let status = child.wait().expect("Failed to wait on Cargo child process");

            if !status.success() {
                log_error(&format!(
                    "[BUILD TOOLS - {}] Error building with wasm-pack",
                    member.path
                ));

                if !member.is_skippable {
                    exit(-1);
                } else {
                    log_info(&format!(
                        "[BUILD TOOLS - {}] Skipping due to error, but continuing execution",
                        member.path
                    ));
                }
            }
        }
    }
}

pub fn build_members(members: Vec<Member>) {
    let start_time = Instant::now();

    let (tx, rx) = channel::bounded::<String>(100); // Change this line

    let logger = std::thread::spawn(move || {
        for received in rx {
            if received == "TERMINATE" {
                // Add this condition
                break;
            }
            println!("{}", received);
        }
    });

    members
        .par_iter()
        .for_each(|member| build_project(member, tx.clone()));

    let end_time = Instant::now();
    let duration = end_time - start_time;

    tx.send("TERMINATE".to_string()).unwrap(); // Add this line

    // Join the logger thread to ensure all logs are printed
    logger.join().unwrap();

    println!("[BUILD TOOLS] Done in  {:?}", duration);
}
