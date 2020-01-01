use serde::Deserialize;

use std::fs;

#[derive(Deserialize)]
pub struct Package {
    pub name: String,
    pub desc: String,
}

pub fn read_pacman_packages() -> Vec<Package> {
    let str_json = fs::read_to_string("/tmp/pacman.json")
        .expect("Something went wrong reading the pacman.json");

    let data: Vec<Package> = serde_json::from_str(&str_json).expect("Deserialize Problem");
    data
}
