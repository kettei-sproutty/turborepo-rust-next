use clap::{Arg, ArgAction, ArgMatches, Command};

fn get_verbosity_arg() -> Arg {
    Arg::new("verbose")
        .long("verbose")
        .short('v')
        .action(ArgAction::SetFalse)
        .value_name("VERBOSE")
        .help("Set the verbosity")
        .required(false)
}

fn get_config_location() -> Arg {
    Arg::new("config")
        .long("config")
        .short('c')
        .value_name("CONFIG")
        .help("Set the config location (default: config.toml)")
        .required(false)
}

fn get_args() -> impl IntoIterator<Item = impl Into<Arg>> {
    let verbosity_arg: Arg = get_verbosity_arg();
    let config_location_arg: Arg = get_config_location();

    [verbosity_arg, config_location_arg]
}

pub fn get_matches() -> ArgMatches {
    let args = get_args();

    Command::new("build_crates")
        .version("0.1.0")
        .author("Alessio Marchi")
        .about("Build task based on build.toml config file")
        .args(args)
        .get_matches()
}
