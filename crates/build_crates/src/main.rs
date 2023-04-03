mod args;
mod build;
mod members;
mod utils;

fn main() {
    let matches = args::get_matches();
    let config_location = matches.get_one("config");

    let effective_members = members::get_members(config_location);
    println!("[BUILD TOOLS] members: {:?}", effective_members);

    build::build_members(effective_members);
}
