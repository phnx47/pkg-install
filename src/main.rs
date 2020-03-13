use arch_post_install::package::{read_packages, Phase};
use colored::*;
use structopt::StructOpt;

//use std::process::Command;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "p", long = "phase")]
    phase: String,
}

fn main() {
    //let phase = read_phase();
    let phase: Phase = Phase::XOrg; // only for debug
    let packages = read_packages(phase);

    for value in packages.iter() {
        println!(
            "{}: {} - {}",
            "Package".blue(),
            value.name.green(),
            value.desc.yellow()
        );
    }

    /* let mut bash_exec = Command::new("bash");
    let apt_install = bash_exec.arg("-c").arg("sudo apt-get install"); // sudo pacman -S "$PKG" --noconfirm --needed
    let packages = package::re;

    for value in packages.iter() {
        println!("Package: {} - {}", value.name.green(), value.desc.blue());

        let output = apt_install.arg(&value.name).output().unwrap();

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    */
}

#[allow(dead_code)]
fn read_phase() -> Phase {
    let args = Cli::from_args();
    let phase_result = args.phase.parse::<Phase>();
    let phase = match phase_result {
        Ok(phase) => phase,
        Err(_) => {
            panic!("Can't parse {:?}", args.phase);
        }
    };

    println!("Phase: {}", phase.to_string().green());
    phase
}
