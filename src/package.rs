use core::fmt;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub desc: String,
}

#[derive(Deserialize)]
struct Config {
    paths: HashMap<String, String>,
}

#[enumeration(case_insensitive)]
#[derive(Debug, Deserialize, enum_utils::FromStr)]
pub enum Phase {
    XOrg,
    SoftPacman,
    SoftCargo,
}

pub fn read_packages(phase: Phase) -> Vec<Package> {
    let config_path = "config.json";
    let config_str_json =
        fs::read_to_string(config_path).expect("Something went wrong reading the config file");

    let config: Config =
        serde_json::from_str(&config_str_json).expect("Config Deserialize Problem");

    let package_file_path: String;
    let str_phase = phase.to_string().to_lowercase();

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

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
