use colored::*;
use indicatif::ProgressBar;
use pkg_install::command::read_command;
use pkg_install::package::read_packages;
use pkg_install::phase::read_phase;

fn main() {
    let phase = read_phase();
    let packages = read_packages(&phase);

    let len = packages.len() as u64;
    let bar = ProgressBar::new(len);

    for value in packages.iter() {
        println!();
        bar.inc(1);
        println!("{} - {}", value.name.green(), value.desc.yellow());
        std::thread::sleep(std::time::Duration::from_millis(300));
        let mut current_command = read_command(&phase);
        let mut child = current_command.arg(&value.name).spawn().unwrap();

        child.wait().unwrap();
    }
    bar.finish();
}
