use crate::consts::*;
use std::fmt::Display;

impl PieceType {
    pub const fn from_index(index: u8) -> Option<Self> {
        if index < 6 {
            // SAFETY: inner is less than 6, so it corresponds to a valid enum variant.
            Some(unsafe { std::mem::transmute::<u8, Self>(index) })
        } else {
            None
        }
    }
    pub const fn to_index(self) -> u8 {
        self as u8
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pawn => write!(f, "Pawn"),
            Self::Knight => write!(f, "Knight"),
            Self::Bishop => write!(f, "Bishop"),
            Self::Rook => write!(f, "Rook"),
            Self::Queen => write!(f, "Queen"),
            Self::King => write!(f, "King"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn from_index() {
        assert_eq!(PieceType::from_index(0), Some(PieceType::Pawn));
        assert_eq!(PieceType::from_index(1), Some(PieceType::Knight));
        assert_eq!(PieceType::from_index(2), Some(PieceType::Bishop));
        assert_eq!(PieceType::from_index(3), Some(PieceType::Rook));
        assert_eq!(PieceType::from_index(4), Some(PieceType::Queen));
        assert_eq!(PieceType::from_index(5), Some(PieceType::King));
        assert_eq!(PieceType::from_index(6), None);
    }
    #[test]
    fn to_index() {
        assert_eq!(PieceType::Pawn.to_index(), 0);
        assert_eq!(PieceType::Knight.to_index(), 1);
        assert_eq!(PieceType::Bishop.to_index(), 2);
        assert_eq!(PieceType::Rook.to_index(), 3);
        assert_eq!(PieceType::Queen.to_index(), 4);
        assert_eq!(PieceType::King.to_index(), 5);
    }
    #[test]
    fn display() {
        assert_eq!(format!("{}", PieceType::Pawn), "Pawn");
        assert_eq!(format!("{}", PieceType::Knight), "Knight");
        assert_eq!(format!("{}", PieceType::Bishop), "Bishop");
        assert_eq!(format!("{}", PieceType::Rook), "Rook");
        assert_eq!(format!("{}", PieceType::Queen), "Queen");
        assert_eq!(format!("{}", PieceType::King), "King");
    }
}
