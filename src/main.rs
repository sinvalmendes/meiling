#![feature(try_from)]
extern crate config;
extern crate clap;

use std::collections::HashMap;
use git2::Repository;
use clap::{Arg, App, SubCommand};

static DEFAULT_CONFIG_FILE_PATH: &str = ".Meiling.toml";
static DEFAULT_REPOSITORY_PATH: &str = "repositories/fixed/";


fn main() {
    let repository_url = get_repository_url();
    println!("{:?}", repository_url);

    let repo = match Repository::open(&DEFAULT_REPOSITORY_PATH) {
        Ok(repo) => repo,
        Err(e) => {
            let cloned_repo = match Repository::clone(&repository_url, &DEFAULT_REPOSITORY_PATH) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to clone: {}", e),
            };
            cloned_repo
        },
    };

    println!("{:?} was opened", repository_url);
    let matches = App::new("meiling")
                          .version("0.0.1")
                          .author("Sinval Vieira <sinvalneto01@gmail.com>")
                          .about("Note manager")
                          .arg(Arg::with_name("create")
                               .short("c")
                               .long("create")
                               .value_name("NOTE_NAME")
                               .help("Create a note")
                               .takes_value(true))
                          .get_matches();

    let note_name = matches.value_of("create").unwrap();
    println!("Note: {}", note_name);

}

fn get_repository_url() -> std::string::String {
    let settings = get_settings();

    match settings.get("repository") {
        Some(x) => format!("{}", x),
        None    => format!("{}", ""),
    }
}

fn get_settings() -> HashMap<String, String> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name(DEFAULT_CONFIG_FILE_PATH)).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();

    settings.try_into::<HashMap<String, String>>().unwrap()
}