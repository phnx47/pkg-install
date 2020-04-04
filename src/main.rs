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

    let mut install_command = match phase {
        Phase::XOrg => {
            let mut pacman = Command::new("pacman");
            pacman.arg("-S").arg("--noconfirm").arg("--needed");
            pacman
        }
        _ => panic!("Can't find program {:?}", phase),
    };

    for value in packages.iter() {
        println!();
        bar.inc(1);
        println!("{} - {}", value.name.green(), value.desc.yellow());
        let mut child = install_command.arg(&value.name).spawn().unwrap();

        // send SIGINT to the child
        nix::sys::signal::kill(
            nix::unistd::Pid::from_raw(child.id() as i32),
            nix::sys::signal::Signal::SIGINT,
        )
        .expect("cannot send ctrl-c");

        // wait for child to terminate
        child.wait().unwrap();
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
