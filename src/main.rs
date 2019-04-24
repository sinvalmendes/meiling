#![feature(try_from)]
extern crate config;

use std::collections::HashMap;
use git2::Repository;


static DEFAULT_CONFIG_FILE_PATH: &str = ".Meiling";


fn main() {
    let repository_url = get_repository_url();
    println!("{:?}", repository_url);

    let repo = match Repository::clone(&repository_url, "repositories/fixed/") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
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