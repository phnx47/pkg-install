use crate::phase::Phase;
use std::process::Command;

pub fn read_command(phase: &Phase) -> Command {
    let install_command = match phase {
        Phase::Xorg | Phase::Kde | Phase::SoftPacman => {
            let mut pacman = Command::new("pacman");
            pacman.arg("-S").arg("--noconfirm").arg("--needed");
            pacman
        }
        Phase::SoftCargo => {
            let mut cargo = Command::new("cargo");
            cargo.arg("install");
            cargo
        }
    };
    install_command
}
