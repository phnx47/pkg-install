use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub desc: String,
}

pub fn read_ubuntu_apt_packages() -> Vec<Package> {
    let file_path = "./packages/ubuntu/apt.json";
    let str_json =
        fs::read_to_string(file_path).expect("Something went wrong reading the config file");

    let data: Vec<Package> = serde_json::from_str(&str_json).expect("Deserialize Problem");
    data
}
