use std::process::Command;

use arch_postinstall::config;

fn main() {
    let mut bash_exec = Command::new("bash");

    read_pacman_packages();

    let apt_install = bash_exec.arg("-c").arg("sudo apt-get install");
    let output = apt_install.arg("nano")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
