use crate::phase::Phase;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use inflector::cases::kebabcase::to_kebab_case;

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub desc: String,
}

#[derive(Deserialize)]
struct Config {
    paths: HashMap<String, String>,
}

pub fn read_packages(phase: &Phase) -> Vec<Package> {
    let config_path = "config.json";
    let config_str_json =
        fs::read_to_string(config_path).expect("Something went wrong reading the config file");

    let config: Config =
        serde_json::from_str(&config_str_json).expect("Config Deserialize Problem");

    let package_file_path: String;
    let str_phase = to_kebab_case(&phase.to_string());

    match config.paths.get(&str_phase) {
        Some(path) => package_file_path = path.parse().unwrap(),
        _ => panic!("Can't find path for: {}", str_phase),
    }

    let package_str_json =
        fs::read_to_string(package_file_path).expect("Something went wrong reading package file");

    let data: Vec<Package> =
        serde_json::from_str(&package_str_json).expect("Packages Deserialize Problem");
    data
}
