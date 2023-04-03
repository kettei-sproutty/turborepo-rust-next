use crate::members::{AnalyzedMember, MemberTarget};
use std::path::Path;
use std::process::{exit, Command};

pub fn build_project(member: &AnalyzedMember) {
    let cargo_toml_path = format!("{}/{}", member.path, "Cargo.toml");
    let is_cargo_project = Path::new(&cargo_toml_path).exists();

    if !is_cargo_project {
        eprintln!("{:?}: is not a cargo project, skipping.", member.path);
        return;
    }

    match &member.target {
        MemberTarget::WasmPack => {
            println!("Building {} with wasm-pack", member.path);
            let mut child = Command::new("wasm-pack")
                .args(["build", &member.path, "--verbose"])
                .spawn()
                .expect("Failed to execute wasm-pack build");

            let status = child.wait().expect("Failed to wait on Cargo child process");

            if !status.success() {
                if member.skippable {
                    eprintln!("{} skipping due error", member.path);
                } else {
                    eprintln!("{} exit due error", member.path);
                    exit(-1);
                }
            }
        }
        MemberTarget::Cargo => {
            // 1. fetchare il Cargo.toml all'interno della path
            // 2. estrapolare dal Cargo.toml il nome del progetto
            // 3. runnare Command (cargo -p nome-progetto)
            // 4. Controllare lo status
            // 5. Se fallisce, controllare se era skippabile, altrimenti ritorna -1
            println!("WIP")
        }
    }
}
