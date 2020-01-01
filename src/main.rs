use std::process::Command;

fn main() {
    let mut bash_exec = Command::new("bash");

    let ss = bash_exec.arg("-c");
    let output = ss.arg("echo hello")
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
