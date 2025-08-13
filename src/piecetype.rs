use crate::consts::*;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

impl PieceType {
    pub const fn new(v: u8) -> Option<Self> {
        if v < 6 {
            // SAFETY: inner is less than 6, so it corresponds to a valid enum variant.
            Some(unsafe { std::mem::transmute::<u8, Self>(v) })
        } else {
            None
        }
    }
    pub const unsafe fn from_index_unchecked(v: u8) -> Self {
        unsafe { std::mem::transmute(v) }
    }
    pub const fn inner(self) -> u8 {
        self as u8
    }
    pub const fn legal_promo(self) -> bool {
        // self == Self::QUEEN || self == Self::KNIGHT || self == Self::BISHOP || self == Self::ROOK
        matches!(self, Self::Queen | Self::Knight | Self::Bishop | Self::Rook)
    }
    pub const fn promo_char(self) -> Option<char> {
        match self {
            Self::Queen => Some('q'),
            Self::Knight => Some('n'),
            Self::Bishop => Some('b'),
            Self::Rook => Some('r'),
            _ => None,
        }
    }
    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        // SAFETY: all values are within `0..6`.
        (0..6u8).map(|i| unsafe { std::mem::transmute(i) })
    }
    pub const fn index(self) -> usize {
        self as usize
    }
    pub fn from_symbol(c: u8) -> Option<Self> {
        const SYMBOLS: [u8; 7] = *b"PNBRQK.";
        SYMBOLS
            .iter()
            .position(|&x| x == c)
            .and_then(|x| Self::new(x.try_into().ok()?))
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
impl<T> Index<PieceType> for [T; 6] {
    type Output = T;

    fn index(&self, index: PieceType) -> &Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked(index as usize) }
    }
}
impl<T> IndexMut<PieceType> for [T; 6] {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn new() {
        assert_eq!(PieceType::new(0), Some(PieceType::Pawn));
        assert_eq!(PieceType::new(1), Some(PieceType::Knight));
        assert_eq!(PieceType::new(2), Some(PieceType::Bishop));
        assert_eq!(PieceType::new(3), Some(PieceType::Rook));
        assert_eq!(PieceType::new(4), Some(PieceType::Queen));
        assert_eq!(PieceType::new(5), Some(PieceType::King));
        assert_eq!(PieceType::new(6), None);
    }
    #[test]
    fn from_index_unchecked() {}
    #[test]
    fn inner() {
        assert_eq!(PieceType::Pawn.inner(), 0);
        assert_eq!(PieceType::Knight.inner(), 1);
        assert_eq!(PieceType::Bishop.inner(), 2);
        assert_eq!(PieceType::Rook.inner(), 3);
        assert_eq!(PieceType::Queen.inner(), 4);
        assert_eq!(PieceType::King.inner(), 5);
    }
    #[test]
    fn legal_promo() {
        assert!(PieceType::Queen.legal_promo());
        assert!(PieceType::Knight.legal_promo());
        assert!(PieceType::Bishop.legal_promo());
        assert!(PieceType::Rook.legal_promo());
        assert!(!PieceType::Pawn.legal_promo());
        assert!(!PieceType::King.legal_promo());
    }
    #[test]
    fn promo_char() {
        assert_eq!(PieceType::Queen.promo_char(), Some('q'));
        assert_eq!(PieceType::Knight.promo_char(), Some('n'));
        assert_eq!(PieceType::Bishop.promo_char(), Some('b'));
        assert_eq!(PieceType::Rook.promo_char(), Some('r'));
        assert_eq!(PieceType::Pawn.promo_char(), None);
        assert_eq!(PieceType::King.promo_char(), None);
    }
    #[test]
    fn all() {
        let mut iter = PieceType::all();
        assert_eq!(iter.next(), Some(PieceType::Pawn));
        assert_eq!(iter.next(), Some(PieceType::Knight));
        assert_eq!(iter.next(), Some(PieceType::Bishop));
        assert_eq!(iter.next(), Some(PieceType::Rook));
        assert_eq!(iter.next(), Some(PieceType::Queen));
        assert_eq!(iter.next(), Some(PieceType::King));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn index() {
        assert_eq!(PieceType::Pawn.index(), 0);
        assert_eq!(PieceType::Knight.index(), 1);
        assert_eq!(PieceType::Bishop.index(), 2);
        assert_eq!(PieceType::Rook.index(), 3);
        assert_eq!(PieceType::Queen.index(), 4);
        assert_eq!(PieceType::King.index(), 5);
    }
    #[test]
    fn from_symbol() {
        assert_eq!(PieceType::from_symbol(b'P'), Some(PieceType::Pawn));
        assert_eq!(PieceType::from_symbol(b'N'), Some(PieceType::Knight));
        assert_eq!(PieceType::from_symbol(b'B'), Some(PieceType::Bishop));
        assert_eq!(PieceType::from_symbol(b'R'), Some(PieceType::Rook));
        assert_eq!(PieceType::from_symbol(b'Q'), Some(PieceType::Queen));
        assert_eq!(PieceType::from_symbol(b'K'), Some(PieceType::King));
        assert_eq!(PieceType::from_symbol(b'.'), None);
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
    #[test]
    fn index_trait() {
        let arr: [u8; 6] = [0, 1, 2, 3, 4, 5];
        assert_eq!(arr[PieceType::Pawn], 0);
        assert_eq!(arr[PieceType::Knight], 1);
        assert_eq!(arr[PieceType::Bishop], 2);
        assert_eq!(arr[PieceType::Rook], 3);
        assert_eq!(arr[PieceType::Queen], 4);
        assert_eq!(arr[PieceType::King], 5);
    }
}
