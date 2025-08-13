use crate::consts::Colour;
use std::fmt::Display;

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::White => write!(f, "w"),
            Self::Black => write!(f, "b"),
        }
    }
}
