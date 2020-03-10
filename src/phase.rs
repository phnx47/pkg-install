use core::fmt;

#[enumeration(case_insensitive)]
#[derive(Debug, enum_utils::FromStr)]
pub enum Phase {
    //todo: try parse from number
    XOrg,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
