use rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;

pub fn read_pacman_packages() {
    let mut file = File::open("./packages/pacman.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let packages: [str] = serde_json::from_str(&buff).unwrap();
    println!("{}", packages);
}