use glob::{glob};
use serde::{Deserialize};
use std::process::exit;

fn default_target() -> String {
    "WasmPack".to_owned()
}

fn default_skippable() -> bool {
    false
}

#[derive(Clone, Debug)]
pub enum MemberTarget {
    WasmPack,
    Cargo,
}

#[derive(Deserialize, Debug, Clone)]
struct Member {
    path: String,
    #[serde(default = "default_target")]
    target: String,
    #[serde(default = "Vec::new")]
    ignore: Vec<String>,
    #[serde(default = "default_skippable")]
    skippable: bool
}

#[derive(Debug, Clone)]
pub struct AnalyzedMember {
    pub path: String,
    pub target: MemberTarget,
    pub skippable: bool
}

#[derive(Deserialize, Debug)]
struct CargoBuild {
    member: Option<Vec<Member>>,
}

fn read_toml(config_location: &str) -> CargoBuild {
    let workspace_toml = std::fs::read_to_string(config_location).unwrap();
    toml::from_str(&workspace_toml).unwrap_or_else(|error| {
        eprintln!("{}", error);
        exit(-1);
    })
}

fn analyze_members(members: Vec<Member>) -> Vec<AnalyzedMember> {
    let analyzed_members: Vec<AnalyzedMember> = members
        .iter()
        .flat_map(|member| {
            let analyzed_members: Vec<AnalyzedMember> = glob(&member.path)
                .unwrap()
                .filter_map(|path| {
                    let path = path.unwrap().to_str().unwrap().to_owned();
                    if member.ignore.contains(&path) {
                        return None;
                    }

                    let target =  if member.target == "WasmPack" {
                        MemberTarget::WasmPack
                    } else {
                        MemberTarget::Cargo
                    };

                    Some(AnalyzedMember {
                        path,
                        target,
                        skippable: member.skippable
                    })
                })
                .collect();

            analyzed_members
        })
        .collect();

    analyzed_members
}

pub fn get_members() -> Vec<AnalyzedMember> {
    let config_location = "Cargo.toml".to_owned();
    let workspace: CargoBuild = read_toml(&config_location);

    let members = workspace.member.unwrap_or_else(|| {
        println!("No members found, exit without errors");
        exit(0)
    });

    let effective_member = analyze_members(members);

    effective_member
}
