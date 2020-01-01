use colored::*;
use post_install::package;
use std::process::Command;

fn main() {
    let mut bash_exec = Command::new("bash");
    let apt_install = bash_exec.arg("-c").arg("sudo apt-get install");
    let packages = package::read_ubuntu_apt_packages();

    for value in packages.iter() {
        println!("Package: {} - {}", value.name.green(), value.desc.blue());

        let output = apt_install.arg(&value.name).output().unwrap();

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
}
