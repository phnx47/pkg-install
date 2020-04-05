use arch_post_install::command::read_command;
use arch_post_install::package::read_packages;
use arch_post_install::phase::read_phase;
use colored::*;
use indicatif::ProgressBar;

fn main() {
    let phase = read_phase();
    let packages = read_packages(&phase);

    let len = packages.capacity() as u64;
    let bar = ProgressBar::new(len);

    let mut install_command = read_command(&phase);

    for value in packages.iter() {
        println!();
        bar.inc(1);
        println!("{} - {}", value.name.green(), value.desc.yellow());
        let mut child = install_command.arg(&value.name).spawn().unwrap();
        // sleep a bit so that child can process the input
        std::thread::sleep(std::time::Duration::from_millis(500));

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
