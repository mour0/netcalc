use std::fmt::{self};


#[derive(Debug, PartialEq)]
pub enum NetHelperError
{
    UnknownClass,
    InvalidCidr,
    InvalidCidrVLSM,
    InvalidIP,
    NoSpace,
}

impl std::error::Error for NetHelperError {}

impl fmt::Display for NetHelperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownClass => write!(f,"Unknown class"),
            Self::InvalidCidr => write!(f,"Invalid CIDR"),
            Self::InvalidCidrVLSM => write!(f,"Invalid CIDR for VLSM [0-30]"),
            Self::InvalidIP => write!(f, "Invalid IP Address"),
            Self::NoSpace => write!(f, "Not enough space"),
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum IPv4Class{
    A,
    B,
    C,
    D,
    E,
}

impl fmt::Display for IPv4Class
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
        }
    }
}