use std::fmt::Write;

use crate::consts::*;

pub const fn contains(bb: BitBoard, other: BitBoard) -> bool {
    (bb & other) == other
}
pub const fn contains_square(bb: BitBoard, square: Square) -> bool {
    let mask = 1u64 << square.index();
    (bb & mask) != 0
}
pub const fn add_square(bb: BitBoard, square: Square) -> BitBoard {
    let mask = 1u64 << square.index();
    bb | mask
}
pub const fn remove_square(bb: BitBoard, square: Square) -> BitBoard {
    let mask = !(1u64 << square.index());
    bb & mask
}
pub const fn toggle_square(bb: BitBoard, square: Square) -> BitBoard {
    let mask = 1u64 << square.index();
    bb ^ mask
}
pub fn iter(bb: BitBoard) -> SquareIter {
    SquareIter::new(bb)
}
pub const fn first(bb: BitBoard) -> Square {
    if bb == 0 {
        return Square::A1; // or any default square
    }
    // SAFETY: u64::trailing_zeros can only return values within `0..64`,
    // all of which correspond to valid enum variants of Square.
    unsafe { Square::new_unchecked(bb.trailing_zeros() as u8) }
}
pub const fn from_square(square: Square) -> BitBoard {
    1u64 << square.index()
}
pub fn north_east_one(bb: BitBoard) -> BitBoard {
    (bb << 9) & !BB_FILE_A
}
pub fn north_west_one(bb: BitBoard) -> BitBoard {
    (bb << 7) & !BB_FILE_H
}
pub fn south_east_one(bb: BitBoard) -> BitBoard {
    (bb >> 7) & !BB_FILE_A
}
pub fn south_west_one(bb: BitBoard) -> BitBoard {
    (bb >> 9) & !BB_FILE_H
}
pub fn east_one(bb: BitBoard) -> BitBoard {
    (bb << 1) & !BB_FILE_A
}
pub fn west_one(bb: BitBoard) -> BitBoard {
    (bb >> 1) & !BB_FILE_H
}
pub fn north_one(bb: BitBoard) -> BitBoard {
    bb << 8
}
pub fn south_one(bb: BitBoard) -> BitBoard {
    bb >> 8
}
pub fn isolate_lsb(bb: BitBoard) -> BitBoard {
    bb & 0u64.wrapping_sub(bb)
}
pub fn without_lsb(bb: BitBoard) -> BitBoard {
    bb & bb.wrapping_sub(1)
}
pub fn one(bb: BitBoard) -> bool {
    bb != BB_EMPTY && without_lsb(bb) == BB_EMPTY
}
pub fn many(bb: BitBoard) -> bool {
    without_lsb(bb) != BB_EMPTY
}
pub fn to_string(bb: BitBoard) -> String {
    let mut s = String::with_capacity(136);
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask = 1u64 << (63 - (rank * 8) - file);
            let char = if bb & mask != 0 { '1' } else { '0' };
            s.write_char(char).unwrap();
            s.write_char(' ').unwrap();
        }
        s.write_char('\n').unwrap();
    }
    s
}

// Iterator over the squares of a square-set.
// The squares are returned in increasing order.
pub struct SquareIter(u64);
impl SquareIter {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}
impl Iterator for SquareIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            // faster if we have bmi (maybe)
            #[allow(clippy::cast_possible_truncation)]
            let lsb: u8 = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1;
            // SAFETY: u64::trailing_zeros can only return values within `0..64`,
            // all of which correspond to valid enum variants of Square.
            Some(unsafe { Square::new_unchecked(lsb) })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bitboard, bitboard::BitBoard, consts::*};
    #[test]
    fn to_string() {
        let bbstr = bitboard::to_string(1);
        let bbstrlen = bbstr.len();
        assert_eq!(bbstrlen, 136);
        assert_eq!(bbstr.starts_with("0 0 0 0 0 0 0 0 \n"), true);
        assert_eq!(bbstr.ends_with("1 0 0 0 0 0 0 0 \n"), true);
    }
    #[test]
    fn empty() {
        let empty: BitBoard = BB_EMPTY;
        assert_eq!(empty, 0);
        assert!(!bitboard::one(empty));
        assert!(!bitboard::many(empty));
    }
    #[test]
    fn full() {
        let full = BB_FULL;
        assert_eq!(full, 18446744073709551615);
        assert!(!bitboard::one(full));
        assert!(bitboard::many(full));
    }
    #[test]
    fn north_east_one() {
        let mut bb = bitboard::from_square(Square::A1);
        let mut ne_bb = bitboard::north_east_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::B2));
        bb = bitboard::from_square(Square::B5);
        ne_bb = bitboard::north_east_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::C6));
        bb = bitboard::from_square(Square::H4);
        ne_bb = bitboard::north_east_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
        bb = bitboard::from_square(Square::G8);
        ne_bb = bitboard::north_east_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
    }
    #[test]
    fn north_west_one() {
        let mut bb = bitboard::from_square(Square::H1);
        let mut ne_bb = bitboard::north_west_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::G2));
        bb = bitboard::from_square(Square::B5);
        ne_bb = bitboard::north_west_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::A6));
        bb = bitboard::from_square(Square::A4);
        ne_bb = bitboard::north_west_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
        bb = bitboard::from_square(Square::G8);
        ne_bb = bitboard::north_west_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
    }
    #[test]
    fn south_east_one() {
        let mut bb = bitboard::from_square(Square::A2);
        let mut ne_bb = bitboard::south_east_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::B1));
        bb = bitboard::from_square(Square::B5);
        ne_bb = bitboard::south_east_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::C4));
        bb = bitboard::from_square(Square::H4);
        ne_bb = bitboard::south_east_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
        bb = bitboard::from_square(Square::G1);
        ne_bb = bitboard::south_east_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
    }
    #[test]
    fn south_west_one() {
        let mut bb = bitboard::from_square(Square::H2);
        let mut ne_bb = bitboard::south_west_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::G1));
        bb = bitboard::from_square(Square::B5);
        ne_bb = bitboard::south_west_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::A4));
        bb = bitboard::from_square(Square::A4);
        ne_bb = bitboard::south_west_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
        bb = bitboard::from_square(Square::G1);
        ne_bb = bitboard::south_west_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
    }
    #[test]
    fn east_one() {
        let mut bb = bitboard::from_square(Square::A2);
        let mut ne_bb = bitboard::east_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::B2));
        bb = bitboard::from_square(Square::B5);
        ne_bb = bitboard::east_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::C5));
        bb = bitboard::from_square(Square::H4);
        ne_bb = bitboard::east_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
    }
    #[test]
    fn west_one() {
        let mut bb = bitboard::from_square(Square::B2);
        let mut ne_bb = bitboard::west_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::A2));
        bb = bitboard::from_square(Square::B5);
        ne_bb = bitboard::west_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::A5));
        bb = bitboard::from_square(Square::A4);
        ne_bb = bitboard::west_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
    }
    #[test]
    fn north_one() {
        let mut bb = bitboard::from_square(Square::B2);
        let mut ne_bb = bitboard::north_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::B3));
        bb = bitboard::from_square(Square::B5);
        ne_bb = bitboard::north_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::B6));
        bb = bitboard::from_square(Square::B8);
        ne_bb = bitboard::north_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
    }
    #[test]
    fn south_one() {
        let mut bb = bitboard::from_square(Square::B2);
        let mut ne_bb = bitboard::south_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::B1));
        bb = bitboard::from_square(Square::B5);
        ne_bb = bitboard::south_one(bb);
        assert_eq!(ne_bb, bitboard::from_square(Square::B4));
        bb = bitboard::from_square(Square::B1);
        ne_bb = bitboard::south_one(bb);
        assert_eq!(ne_bb, BB_EMPTY);
    }
    #[test]
    fn isolate_lsb() {
        let bb = BB_FILE_A;
        let isolated = bitboard::isolate_lsb(bb);
        assert_eq!(isolated, bitboard::from_square(Square::A1));
    }
    #[test]
    fn without_lsb() {
        let bb = bitboard::from_square(Square::A2) | bitboard::from_square(Square::A3);
        let without_lsb = bitboard::without_lsb(bb);
        assert_eq!(without_lsb, bitboard::from_square(Square::A3));
    }
    #[test]
    fn add_square() {
        let one = Square::E4.as_bb();
        assert_ne!(one, BB_EMPTY);
        assert!(bitboard::one(one));
        assert!(!bitboard::many(one));

        let two = bitboard::add_square(one, Square::E5);
        assert_ne!(two, BB_EMPTY);
        assert!(!bitboard::one(two));
        assert!(bitboard::many(two));
    }
    #[test]
    fn iter() {
        let bb = BB_FILE_A;
        let mut bb_it = bitboard::iter(bb);
        assert_eq!(bb_it.next(), Some(Square::A1));
        assert_eq!(bb_it.next(), Some(Square::A2));
        bb_it = bitboard::iter(BB_FILE_B);
        let bstr: Vec<String> = bb_it.map(|s| s.to_string()).collect();
        assert_eq!(bstr, vec!["b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8"]);
    }
}
