#![feature(try_from)]
extern crate config;

use std::collections::HashMap;

static DEFAULT_CONFIG_FILE_PATH: &str = ".Meiling";


fn main() {
    let repository_address = get_repository_address();
    println!("{:?}", repository_address);
}

fn get_repository_address() -> std::string::String {
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