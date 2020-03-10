//use colored::*;
//use post_install::package;
//use std::process::Command;
use arch_post_install::phase::Phase;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "p", long = "phase")]
    phase: String,
}

fn main() {
    let args = Cli::from_args();
    let phase_result = args.phase.parse::<Phase>();
    let phase = match phase_result {
        Ok(phase) => phase,
        Err(_) => {
            panic!("Can't parse {:?}", args.phase);
        }
    };
    println!("{:?}", phase);

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
