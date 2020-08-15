use clap::{App, Arg, ArgMatches};

pub fn setup_cmd_program_arguments() -> ArgMatches<'static> {
    App::new("Chip-8 VM emulator")
        .version("0.1.0")
        .author("Jakub Sordyl 'Mapet13' <jakubsordyl1@gmail.com>")
        .about("This is a simple Chip-8 VM emulator developed in Rust-lang for learning purpose.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .required(true)
                .takes_value(true)
                .help("The ROM file you want to run in this VM"),
        )
        .get_matches()
}

pub fn get_rom_path(matches: ArgMatches) -> Result<String, String> {
    match matches.value_of("file") {
        Some(value) => Ok(value.to_string()),
        None => Err("Command line argument error".to_string()),
    }
}