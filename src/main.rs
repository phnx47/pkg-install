use arch_post_install::package::{read_packages, Phase};
use colored::*;
use indicatif::ProgressBar;
use std::process::Command;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "p", long = "phase")]
    phase: String,
}

fn main() {
    let phase = read_phase();
    //let phase: Phase = Phase::XOrg; // only for debug
    let packages = read_packages(&phase);

    let len = packages.capacity() as u64;
    let bar = ProgressBar::new(len);
    let mut install_command: Command = Command::new("bash");

    match phase {
        Phase::XOrg => install_command
            .arg("-c")
            .arg("sudo pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg("--needed"),
        _ => panic!("Can't find command {:?}", phase),
    };

    for value in packages.iter() {
        println!();
        println!("{} - {}", value.name.green(), value.desc.yellow());
        let output = install_command
            .arg(&value.name)
            .output()
            .ok()
            .expect("Failed to execute.");

        let std_out = String::from_utf8_lossy(&output.stdout);
        let std_err = String::from_utf8_lossy(&output.stderr);

        if !std_out.is_empty() {
            println!("{}", std_out.bold());
        }

        if !std_err.is_empty() {
            println!("{}", std_err.red());
        }

        bar.inc(1);
    }
    bar.finish();
}

//#[allow(dead_code)]
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
