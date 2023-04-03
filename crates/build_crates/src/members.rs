use colored::*;
use glob::glob;
use std::process::exit;
use toml::Value;

#[derive(Debug, Clone)]
pub enum MemberTarget {
    Cargo,
    WasmPack,
}

#[derive(Debug, Clone)]
pub struct Member {
    pub path: String,
    pub target: MemberTarget,
    pub is_expandable: bool,
    pub is_skippable: bool,
}

fn read_toml(config_location: &str) -> Value {
    let workspace_toml = std::fs::read_to_string(config_location).unwrap();
    toml::from_str::<Value>(&workspace_toml).unwrap()
}

fn parse_members(workspace: &Value) -> Vec<Member> {
    workspace["member"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|member| {
            member.as_table().map(|table| {
                let path = table
                    .get("path")
                    .unwrap_or_else(|| {
                        eprintln!(
                            "{} {}",
                            "[BUILD TOOLS]".red(),
                            "path cannot be undefined".white()
                        );
                        exit(-1)
                    })
                    .as_str()
                    .unwrap_or_else(|| {
                        eprintln!(
                            "{} {}",
                            "[BUILD TOOLS]".red(),
                            "path cannot be converted as string".white()
                        );
                        exit(-1)
                    })
                    .to_string();

                let target = match table.get("target") {
                    Some(value) => {
                        if value.as_str().unwrap() == "WasmPack" {
                            MemberTarget::WasmPack
                        } else {
                            MemberTarget::Cargo
                        }
                    }
                    None => MemberTarget::Cargo,
                };

                let is_expandable = path.trim().ends_with('*');

                let is_skippable = match table.get("skippable") {
                    Some(value) => value.as_bool().unwrap_or(false),
                    None => false,
                };

                Member {
                    path,
                    target,
                    is_expandable,
                    is_skippable,
                }
            })
        })
        .collect::<Vec<_>>()
}

fn parse_ignored_members(workspace: &Value) -> Vec<String> {
    workspace
        .get("ignored_member")
        .and_then(|v| v.as_array())
        .map(|members| {
            members
                .iter()
                .map(|m| {
                    let ignored_path = m["path"].as_str().unwrap_or_default().to_string();
                    ignored_path
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new)
}

fn expand_members(members: Vec<Member>) -> Vec<Member> {
    members
        .into_iter()
        .flat_map(|member| {
            if !member.is_expandable {
                return vec![member].into_iter();
            }

            let expanded_members: Vec<Member> = glob(&member.path)
                .unwrap_or_else(|_| {
                    eprintln!(
                        "{} {}",
                        "[BUILD TOOLS]".red(),
                        format!("Cannot expand path: {}", member.path).white()
                    );
                    exit(-1);
                })
                .filter_map(Result::ok)
                .map(|path| {
                    let new_path = path.to_string_lossy().into_owned();

                    Member {
                        path: new_path,
                        target: member.target.clone(),
                        is_expandable: false,
                        is_skippable: member.is_skippable,
                    }
                })
                .collect::<Vec<_>>();

            expanded_members.into_iter()
        })
        .collect()
}

pub fn get_members(config_location: Option<&String>) -> Vec<Member> {
    let default_config_value = "build.toml".to_owned();
    let config_location = config_location.unwrap_or(&default_config_value);

    let workspace = read_toml(config_location);

    let members = parse_members(&workspace);
    let ignored_members = parse_ignored_members(&workspace);

    members
        .into_iter()
        .flat_map(|member| expand_members(vec![member]))
        .filter(|member| !ignored_members.contains(&member.path))
        .collect()
}
