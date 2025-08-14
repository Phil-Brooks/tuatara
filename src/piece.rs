use crate::consts::*;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

impl Piece {
    pub const fn new(colour: Col, piece_type: PieceType) -> Self {
        let index = colour as u8 * 6 + piece_type as u8;
        // SAFETY: Col is {0, 1}, piece_type is {0, 1, 2, 3, 4, 5}.
        // colour * 6 + piece_type is therefore at most 11, which corresponds
        // to a valid enum variant.
        unsafe { std::mem::transmute(index) }
    }
    pub const fn from_index(v: u8) -> Option<Self> {
        if v < 12 {
            // SAFETY: inner is less than 12, so it corresponds to a valid enum variant.
            Some(unsafe { std::mem::transmute::<u8, Self>(v) })
        } else {
            None
        }
    }
    pub const fn colour(self) -> Col {
        if (self as u8) < 6 {
            Col::White
        } else {
            Col::Black
        }
    }
    pub const fn piecetype(self) -> PieceType {
        let pt_index = self as u8 % 6;
        // SAFETY: pt_index is always within the bounds of the type.
        unsafe { PieceType::from_index_unchecked(pt_index) }
    }
    pub const fn from_piecetype_and_col(piecetype: PieceType, colour: Col) -> Self {
        let index = colour as u8 * 6 + piecetype as u8;
        unsafe { std::mem::transmute(index) }
    }
    pub const fn char(self) -> char {
        match self {
            Self::WP => 'P',
            Self::WN => 'N',
            Self::WB => 'B',
            Self::WR => 'R',
            Self::WQ => 'Q',
            Self::WK => 'K',
            Self::BP => 'p',
            Self::BN => 'n',
            Self::BB => 'b',
            Self::BR => 'r',
            Self::BQ => 'q',
            Self::BK => 'k',
        }
    }
    pub fn byte_char(self) -> u8 {
        b"PNBRQKpnbrqk"[self]
    }
    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        // SAFETY: all values are within `0..12`.
        (0..12u8).map(|i| unsafe { std::mem::transmute(i) })
    }
    pub const fn inner(self) -> u8 {
        self as u8
    }
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}
impl<T> Index<Piece> for [T; 12] {
    type Output = T;

    fn index(&self, index: Piece) -> &Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked(index as usize) }
    }
}
impl<T> IndexMut<Piece> for [T; 12] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn new() {
        assert_eq!(Piece::new(Col::White, PieceType::Pawn), Piece::WP);
        assert_eq!(Piece::new(Col::Black, PieceType::Pawn), Piece::BP);
        assert_eq!(Piece::new(Col::White, PieceType::Knight), Piece::WN);
        assert_eq!(Piece::new(Col::Black, PieceType::Knight), Piece::BN);
        assert_eq!(Piece::new(Col::White, PieceType::Bishop), Piece::WB);
        assert_eq!(Piece::new(Col::Black, PieceType::Bishop), Piece::BB);
        assert_eq!(Piece::new(Col::White, PieceType::Rook), Piece::WR);
        assert_eq!(Piece::new(Col::Black, PieceType::Rook), Piece::BR);
        assert_eq!(Piece::new(Col::White, PieceType::Queen), Piece::WQ);
        assert_eq!(Piece::new(Col::Black, PieceType::Queen), Piece::BQ);
        assert_eq!(Piece::new(Col::White, PieceType::King), Piece::WK);
        assert_eq!(Piece::new(Col::Black, PieceType::King), Piece::BK);
    }
    #[test]
    fn from_index() {
        assert_eq!(Piece::from_index(0), Some(Piece::WP));
        assert_eq!(Piece::from_index(1), Some(Piece::WN));
        assert_eq!(Piece::from_index(2), Some(Piece::WB));
        assert_eq!(Piece::from_index(3), Some(Piece::WR));
        assert_eq!(Piece::from_index(4), Some(Piece::WQ));
        assert_eq!(Piece::from_index(5), Some(Piece::WK));
        assert_eq!(Piece::from_index(6), Some(Piece::BP));
        assert_eq!(Piece::from_index(7), Some(Piece::BN));
        assert_eq!(Piece::from_index(8), Some(Piece::BB));
        assert_eq!(Piece::from_index(9), Some(Piece::BR));
        assert_eq!(Piece::from_index(10), Some(Piece::BQ));
        assert_eq!(Piece::from_index(11), Some(Piece::BK));
        assert_eq!(Piece::from_index(12), None);
    }
    #[test]
    fn colour() {
        assert_eq!(Piece::WP.colour(), Col::White);
        assert_eq!(Piece::BP.colour(), Col::Black);
        assert_eq!(Piece::WN.colour(), Col::White);
        assert_eq!(Piece::BN.colour(), Col::Black);
        assert_eq!(Piece::WB.colour(), Col::White);
        assert_eq!(Piece::BB.colour(), Col::Black);
        assert_eq!(Piece::WR.colour(), Col::White);
        assert_eq!(Piece::BR.colour(), Col::Black);
        assert_eq!(Piece::WQ.colour(), Col::White);
        assert_eq!(Piece::BQ.colour(), Col::Black);
        assert_eq!(Piece::WK.colour(), Col::White);
        assert_eq!(Piece::BK.colour(), Col::Black);
    }
    #[test]
    fn piecetype() {
        assert_eq!(Piece::WP.piecetype(), PieceType::Pawn);
        assert_eq!(Piece::BP.piecetype(), PieceType::Pawn);
        assert_eq!(Piece::WN.piecetype(), PieceType::Knight);
        assert_eq!(Piece::BN.piecetype(), PieceType::Knight);
        assert_eq!(Piece::WB.piecetype(), PieceType::Bishop);
        assert_eq!(Piece::BB.piecetype(), PieceType::Bishop);
        assert_eq!(Piece::WR.piecetype(), PieceType::Rook);
        assert_eq!(Piece::BR.piecetype(), PieceType::Rook);
        assert_eq!(Piece::WQ.piecetype(), PieceType::Queen);
        assert_eq!(Piece::BQ.piecetype(), PieceType::Queen);
        assert_eq!(Piece::WK.piecetype(), PieceType::King);
        assert_eq!(Piece::BK.piecetype(), PieceType::King);
    }
    #[test]
    fn from_piecetype_and_col() {
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Pawn, Col::White),
            Piece::WP
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Pawn, Col::Black),
            Piece::BP
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Knight, Col::White),
            Piece::WN
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Knight, Col::Black),
            Piece::BN
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Bishop, Col::White),
            Piece::WB
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Bishop, Col::Black),
            Piece::BB
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Rook, Col::White),
            Piece::WR
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Rook, Col::Black),
            Piece::BR
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Queen, Col::White),
            Piece::WQ
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::Queen, Col::Black),
            Piece::BQ
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::King, Col::White),
            Piece::WK
        );
        assert_eq!(
            Piece::from_piecetype_and_col(PieceType::King, Col::Black),
            Piece::BK
        );
    }
    #[test]
    fn char() {
        assert_eq!(Piece::WP.char(), 'P');
        assert_eq!(Piece::BP.char(), 'p');
        assert_eq!(Piece::WN.char(), 'N');
        assert_eq!(Piece::BN.char(), 'n');
        assert_eq!(Piece::WB.char(), 'B');
        assert_eq!(Piece::BB.char(), 'b');
        assert_eq!(Piece::WR.char(), 'R');
        assert_eq!(Piece::BR.char(), 'r');
        assert_eq!(Piece::WQ.char(), 'Q');
        assert_eq!(Piece::BQ.char(), 'q');
        assert_eq!(Piece::WK.char(), 'K');
        assert_eq!(Piece::BK.char(), 'k');
    }
    #[test]
    fn byte_char() {
        assert_eq!(Piece::WP.byte_char(), b'P');
        assert_eq!(Piece::BP.byte_char(), b'p');
        assert_eq!(Piece::WN.byte_char(), b'N');
        assert_eq!(Piece::BN.byte_char(), b'n');
        assert_eq!(Piece::WB.byte_char(), b'B');
        assert_eq!(Piece::BB.byte_char(), b'b');
        assert_eq!(Piece::WR.byte_char(), b'R');
        assert_eq!(Piece::BR.byte_char(), b'r');
        assert_eq!(Piece::WQ.byte_char(), b'Q');
        assert_eq!(Piece::BQ.byte_char(), b'q');
        assert_eq!(Piece::WK.byte_char(), b'K');
        assert_eq!(Piece::BK.byte_char(), b'k');
    }
    #[test]
    fn all() {
        let mut iter = Piece::all();
        assert_eq!(iter.next(), Some(Piece::WP));
        assert_eq!(iter.next(), Some(Piece::WN));
        assert_eq!(iter.next(), Some(Piece::WB));
        assert_eq!(iter.next(), Some(Piece::WR));
        assert_eq!(iter.next(), Some(Piece::WQ));
        assert_eq!(iter.next(), Some(Piece::WK));
        assert_eq!(iter.next(), Some(Piece::BP));
        assert_eq!(iter.next(), Some(Piece::BN));
        assert_eq!(iter.next(), Some(Piece::BB));
        assert_eq!(iter.next(), Some(Piece::BR));
        assert_eq!(iter.next(), Some(Piece::BQ));
        assert_eq!(iter.next(), Some(Piece::BK));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn inner() {
        assert_eq!(Piece::WP.inner(), 0);
        assert_eq!(Piece::WN.inner(), 1);
        assert_eq!(Piece::WB.inner(), 2);
        assert_eq!(Piece::WR.inner(), 3);
        assert_eq!(Piece::WQ.inner(), 4);
        assert_eq!(Piece::WK.inner(), 5);
        assert_eq!(Piece::BP.inner(), 6);
        assert_eq!(Piece::BN.inner(), 7);
        assert_eq!(Piece::BB.inner(), 8);
        assert_eq!(Piece::BR.inner(), 9);
        assert_eq!(Piece::BQ.inner(), 10);
        assert_eq!(Piece::BK.inner(), 11);
    }
    #[test]
    fn index_trait() {
        let pieces: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(pieces[Piece::WP], 0);
        assert_eq!(pieces[Piece::WN], 1);
        assert_eq!(pieces[Piece::WB], 2);
        assert_eq!(pieces[Piece::WR], 3);
        assert_eq!(pieces[Piece::WQ], 4);
        assert_eq!(pieces[Piece::WK], 5);
        assert_eq!(pieces[Piece::BP], 6);
        assert_eq!(pieces[Piece::BN], 7);
        assert_eq!(pieces[Piece::BB], 8);
        assert_eq!(pieces[Piece::BR], 9);
        assert_eq!(pieces[Piece::BQ], 10);
        assert_eq!(pieces[Piece::BK], 11);
    }
    #[test]
    fn display() {
        assert_eq!(format!("{}", Piece::WP), "P");
        assert_eq!(format!("{}", Piece::BP), "p");
        assert_eq!(format!("{}", Piece::WN), "N");
        assert_eq!(format!("{}", Piece::BN), "n");
        assert_eq!(format!("{}", Piece::WB), "B");
        assert_eq!(format!("{}", Piece::BB), "b");
        assert_eq!(format!("{}", Piece::WR), "R");
        assert_eq!(format!("{}", Piece::BR), "r");
        assert_eq!(format!("{}", Piece::WQ), "Q");
        assert_eq!(format!("{}", Piece::BQ), "q");
        assert_eq!(format!("{}", Piece::WK), "K");
        assert_eq!(format!("{}", Piece::BK), "k");
    }
}
