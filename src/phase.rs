use colored::*;
use core::fmt;
use inflector::cases::titlecase::to_title_case;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    #[structopt(short = "p", long = "phase")]
    phase: String,
}

#[enumeration(case_insensitive)]
#[enumeration(rename_all = "kebab-case")]
#[derive(Debug, Deserialize, enum_utils::FromStr)]
pub enum Phase {
    Video,
    Kde,
    SoftPacman,
    SoftAur,
    SoftCargo,
    SoftFlatpak,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn read_phase() -> Phase {
    let args = Cli::from_args();
    let phase_result = args.phase.parse::<Phase>();
    let phase = match phase_result {
        Ok(phase) => phase,
        Err(_) => {
            panic!("Can't parse Phase {:?}", args.phase);
        }
    };

    println!("Phase: {}", to_title_case(&args.phase).green());
    phase
}
