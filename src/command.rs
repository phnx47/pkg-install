use crate::phase::Phase;
use std::process::Command;

pub fn read_command(phase: &Phase) -> Command {
    let install_command = match phase {
        Phase::Video | Phase::Kde | Phase::Pacman => {
            let mut pacman = Command::new("pacman");
            pacman.arg("-S").arg("--noconfirm").arg("--needed");
            pacman
        }
        Phase::Aur => {
            let mut rua = Command::new("rua");
            rua.arg("install");
            rua
        }
        Phase::Cargo => {
            let mut cargo = Command::new("cargo");
            cargo.arg("install");
            cargo
        }
        Phase::Flatpak => {
            let mut flatpak = Command::new("flatpak");
            flatpak.arg("install");
            flatpak
        }
        Phase::Npm => {
            let mut npm = Command::new("npm");
            npm.arg("install").arg("-g");
            npm
        }
    };
    install_command
}
