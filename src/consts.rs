use crate::bitboard::BitBoard;
use crate::colour::Colour;
use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;

const _COLOUR_ASSERT: () = assert!(size_of::<Colour>() == size_of::<Option<Colour>>());
const _FILE_ASSERT: () = assert!(size_of::<File>() == size_of::<Option<File>>());
const _RANK_ASSERT: () = assert!(size_of::<Rank>() == size_of::<Option<Rank>>());
const _SQUARE_ASSERT: () = assert!(size_of::<Square>() == size_of::<Option<Square>>());

const SQUARE_NAMES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

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
    BB_RANK_1,
    BB_RANK_2,
    BB_RANK_3,
    BB_RANK_4,
    BB_RANK_5,
    BB_RANK_6,
    BB_RANK_7,
    BB_RANK_8,
];
pub const FILES: [BitBoard; 8] = [
    BB_FILE_A,
    BB_FILE_B,
    BB_FILE_C,
    BB_FILE_D,
    BB_FILE_E,
    BB_FILE_F,
    BB_FILE_G,
    BB_FILE_H,
];


