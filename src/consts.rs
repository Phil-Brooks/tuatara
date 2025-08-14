const _COLOUR_ASSERT: () = assert!(size_of::<Colour>() == size_of::<Option<Colour>>());
const _FILE_ASSERT: () = assert!(size_of::<File>() == size_of::<Option<File>>());
const _RANK_ASSERT: () = assert!(size_of::<Rank>() == size_of::<Option<Rank>>());
const _SQUARE_ASSERT: () = assert!(size_of::<Square>() == size_of::<Option<Square>>());
const _PIECE_TYPE_ASSERT: () = assert!(size_of::<PieceType>() == size_of::<Option<PieceType>>());
const _PIECE_ASSERT: () = assert!(size_of::<Piece>() == size_of::<Option<Piece>>());

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Colour {
    White,
    Black,
}
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}
#[rustfmt::skip]
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Debug, Default)]
#[repr(u8)]
pub enum Square {
    #[default]
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

pub const SQUARE_NAMES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];
pub type BitBoard = u64;
pub const BB_EMPTY: BitBoard = 0;
pub const BB_FULL: BitBoard = !0;
pub const BB_RANK_1: BitBoard = 0x0000_0000_0000_00FF;
pub const BB_RANK_2: BitBoard = 0x0000_0000_0000_FF00;
pub const BB_RANK_3: BitBoard = 0x0000_0000_00FF_0000;
pub const BB_RANK_4: BitBoard = 0x0000_0000_FF00_0000;
pub const BB_RANK_5: BitBoard = 0x0000_00FF_0000_0000;
pub const BB_RANK_6: BitBoard = 0x0000_FF00_0000_0000;
pub const BB_RANK_7: BitBoard = 0x00FF_0000_0000_0000;
pub const BB_RANK_8: BitBoard = 0xFF00_0000_0000_0000;
pub const BB_FILE_A: BitBoard = 0x0101_0101_0101_0101;
pub const BB_FILE_B: BitBoard = 0x0202_0202_0202_0202;
pub const BB_FILE_C: BitBoard = 0x0404_0404_0404_0404;
pub const BB_FILE_D: BitBoard = 0x0808_0808_0808_0808;
pub const BB_FILE_E: BitBoard = 0x1010_1010_1010_1010;
pub const BB_FILE_F: BitBoard = 0x2020_2020_2020_2020;
pub const BB_FILE_G: BitBoard = 0x4040_4040_4040_4040;
pub const BB_FILE_H: BitBoard = 0x8080_8080_8080_8080;
pub const BB_LIGHT_SQUARES: BitBoard = 0x55AA_55AA_55AA_55AA;
pub const BB_DARK_SQUARES: BitBoard = 0xAA55_AA55_AA55_AA55;
pub const RANKS: [BitBoard; 8] = [
    BB_RANK_1, BB_RANK_2, BB_RANK_3, BB_RANK_4, BB_RANK_5, BB_RANK_6, BB_RANK_7, BB_RANK_8,
];
pub const FILES: [BitBoard; 8] = [
    BB_FILE_A, BB_FILE_B, BB_FILE_C, BB_FILE_D, BB_FILE_E, BB_FILE_F, BB_FILE_G, BB_FILE_H,
];
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
#[rustfmt::skip]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
#[repr(u8)]
pub enum Piece {
    #[default]
    WP, WN, WB, WR, WQ, WK,
    BP, BN, BB, BR, BQ, BK,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Board {
    pub by_piece: [BitBoard; 6],
    pub by_col: [BitBoard; 2],
    pub occupied: BitBoard,
}
#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn bitboard_constants() {
        assert_eq!(BB_EMPTY, 0);
        assert_eq!(BB_FULL, !0);
        assert_eq!(BB_RANK_1, 0x0000_0000_0000_00FF);
        assert_eq!(BB_FILE_A, 0x0101_0101_0101_0101);
    }
    #[test]
    fn square_names() {
        assert_eq!(SQUARE_NAMES[0], "a1");
        assert_eq!(SQUARE_NAMES[63], "h8");
    }
    #[test]
    fn piece_type_constants() {
        assert_eq!(PieceType::Pawn as u8, 0);
        assert_eq!(PieceType::Knight as u8, 1);
        assert_eq!(PieceType::Bishop as u8, 2);
        assert_eq!(PieceType::Rook as u8, 3);
        assert_eq!(PieceType::Queen as u8, 4);
        assert_eq!(PieceType::King as u8, 5);
    }
    #[test]
    fn piece_constants() {
        assert_eq!(Piece::WP as u8, 0);
        assert_eq!(Piece::WN as u8, 1);
        assert_eq!(Piece::WB as u8, 2);
        assert_eq!(Piece::WR as u8, 3);
        assert_eq!(Piece::WQ as u8, 4);
        assert_eq!(Piece::WK as u8, 5);
        assert_eq!(Piece::BP as u8, 6);
        assert_eq!(Piece::BN as u8, 7);
        assert_eq!(Piece::BB as u8, 8);
        assert_eq!(Piece::BR as u8, 9);
        assert_eq!(Piece::BQ as u8, 10);
        assert_eq!(Piece::BK as u8, 11);
    }
}
