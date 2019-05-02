#![feature(try_from)]
extern crate config;
extern crate clap;

use std::collections::HashMap;
use std::io;
use std::io::{Write};
use std::process::Command;
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
                          .subcommand(SubCommand::with_name("push")
                                .about("push current state"))
                          .subcommand(SubCommand::with_name("status")
                                .about("get current state"))
                          .subcommand(SubCommand::with_name("pull")
                                .about("pull from repository"))
                          .get_matches();

    let create = matches.value_of("create").unwrap_or("no_value");
    if create != "no_value" {
        println!("Creating...");

        let note_name = matches.value_of("create").unwrap();
        println!("Note: {}", note_name);

        let vim = "/usr/bin/vim";
        let note_file_path = format!("{}{}", &DEFAULT_REPOSITORY_PATH, note_name);
        match Command::new(vim).arg(&note_file_path).status() {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error: Unable to open file [{}] with vim [{}]: {}", vim, &note_file_path, e);
                Err(e)
            }
        };
    }

    if let Some(matches) = matches.subcommand_matches("push") {
        println!("Pushing...");
    }

    if let Some(matches) = matches.subcommand_matches("status") {
        git_status();
    }

    if let Some(matches) = matches.subcommand_matches("pull") {
        println!("Pulling...");
    }
}

fn git_status() {
    let output = Command::new("git").args(&["status", &DEFAULT_REPOSITORY_PATH])
        .output().expect("failed to execute process");

    let string = String::from_utf8_lossy(&output.stdout);
    let vec: Vec<&str> = string.split("\n").collect();
    for line in vec {
        println!("{}", line);
    }
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