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
    //let phase = read_phase();
    let phase: Phase = Phase::XOrg; // only for debug
    let packages = read_packages(&phase);

    let len = packages.capacity() as u64;
    let bar = ProgressBar::new(len);
    let mut bash_exec = Command::new("bash");

    let install_command = match phase {
        Phase::XOrg => bash_exec
            .arg("-c")
            .arg("sudo pacman -S --noconfirm --needed"),
        _ => panic!("Can't find command {:?}", phase),
    };

    for value in packages.iter() {
        println!();
        println!(
            "{}: {} - {}",
            "Package".blue(),
            value.name.green(),
            value.desc.yellow()
        );
        let output = install_command.arg(&value.name).output().unwrap();

        let std_out = String::from_utf8_lossy(&output.stdout);
        let std_err = String::from_utf8_lossy(&output.stdout);

        if !std_out.is_empty() {
            println!("{}", std_out);
        }

        if !std_err.is_empty() {
            println!("{}", std_err.red());
        }

        bar.inc(1);
    }
    bar.finish();

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
