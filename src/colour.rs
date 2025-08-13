use crate::consts::Colour;
use std::fmt::Display;

impl Colour {
    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::White),
            1 => Some(Self::Black),
            _ => None,
        }
    }
    pub const fn to_u8(self) -> u8 {
        match self {
            Self::White => 0,
            Self::Black => 1,
        }
    }
    pub const fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::White => write!(f, "w"),
            Self::Black => write!(f, "b"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn from_u8() {
        assert_eq!(Colour::from_u8(0), Some(Colour::White));
        assert_eq!(Colour::from_u8(1), Some(Colour::Black));
        assert_eq!(Colour::from_u8(2), None);
    }
    #[test]
    fn to_u8() {
        assert_eq!(Colour::White.to_u8(), 0);
        assert_eq!(Colour::Black.to_u8(), 1);
    }
    #[test]
    fn opposite() {
        assert_eq!(Colour::White.opposite(), Colour::Black);
        assert_eq!(Colour::Black.opposite(), Colour::White);
    }
}
