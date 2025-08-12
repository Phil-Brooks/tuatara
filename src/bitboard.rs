use std::{
    fmt::{Display, Write},
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr, Sub,
        SubAssign,
    },
};

use crate::types::Square;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
#[repr(transparent)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(!0);
    pub const RANK_1: Self = Self(0x0000_0000_0000_00FF);
    pub const RANK_2: Self = Self(0x0000_0000_0000_FF00);
    pub const RANK_3: Self = Self(0x0000_0000_00FF_0000);
    pub const RANK_4: Self = Self(0x0000_0000_FF00_0000);
    pub const RANK_5: Self = Self(0x0000_00FF_0000_0000);
    pub const RANK_6: Self = Self(0x0000_FF00_0000_0000);
    pub const RANK_7: Self = Self(0x00FF_0000_0000_0000);
    pub const RANK_8: Self = Self(0xFF00_0000_0000_0000);
    pub const FILE_A: Self = Self(0x0101_0101_0101_0101);
    pub const FILE_B: Self = Self(0x0202_0202_0202_0202);
    pub const FILE_C: Self = Self(0x0404_0404_0404_0404);
    pub const FILE_D: Self = Self(0x0808_0808_0808_0808);
    pub const FILE_E: Self = Self(0x1010_1010_1010_1010);
    pub const FILE_F: Self = Self(0x2020_2020_2020_2020);
    pub const FILE_G: Self = Self(0x4040_4040_4040_4040);
    pub const FILE_H: Self = Self(0x8080_8080_8080_8080);
    pub const LIGHT_SQUARES: Self = Self(0x55AA_55AA_55AA_55AA);
    pub const DARK_SQUARES: Self = Self(0xAA55_AA55_AA55_AA55);
    pub const RANKS: [Self; 8] = [
        Self::RANK_1,
        Self::RANK_2,
        Self::RANK_3,
        Self::RANK_4,
        Self::RANK_5,
        Self::RANK_6,
        Self::RANK_7,
        Self::RANK_8,
    ];
    pub const FILES: [Self; 8] = [
        Self::FILE_A,
        Self::FILE_B,
        Self::FILE_C,
        Self::FILE_D,
        Self::FILE_E,
        Self::FILE_F,
        Self::FILE_G,
        Self::FILE_H,
    ];
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const fn contains_square(self, square: Square) -> bool {
        let mask = 1u64 << square.index();
        (self.0 & mask) != 0
    }
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
    pub const fn add_square(self, square: Square) -> Self {
        let mask = 1u64 << square.index();
        Self(self.0 | mask)
    }
    pub const fn remove(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }
    pub const fn remove_square(self, square: Square) -> Self {
        let mask = !(1u64 << square.index());
        Self(self.0 & mask)
    }
    pub const fn toggle(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }
    pub const fn toggle_square(self, square: Square) -> Self {
        let mask = 1u64 << square.index();
        Self(self.0 ^ mask)
    }
    //#[allow(clippy::missing_const_for_fn)]
    pub fn iter(self) -> SquareIter {
        SquareIter::new(self.0)
    }
    //#[allow(clippy::cast_possible_truncation)]
    pub const fn first(self) -> Square {
        if self.0 == 0 {
            return Square::A1; // or any default square
        }
        // SAFETY: u64::trailing_zeros can only return values within `0..64`,
        // all of which correspond to valid enum variants of Square.
        unsafe { Square::new_unchecked(self.0.trailing_zeros() as u8) }
    }
    pub const fn from_square(square: Square) -> Self {
        Self(1u64 << square.index())
    }
    pub fn north_east_one(self) -> Self {
        (self << 9) & !Self::FILE_A
    }
    pub fn north_west_one(self) -> Self {
        (self << 7) & !Self::FILE_H
    }
    pub fn south_east_one(self) -> Self {
        (self >> 7) & !Self::FILE_A
    }
    pub fn south_west_one(self) -> Self {
        (self >> 9) & !Self::FILE_H
    }
    pub fn east_one(self) -> Self {
        (self << 1) & !Self::FILE_A
    }
    pub fn west_one(self) -> Self {
        (self >> 1) & !Self::FILE_H
    }
    pub fn north_one(self) -> Self {
        self << 8
    }
    pub fn south_one(self) -> Self {
        self >> 8
    }
    pub fn isolate_lsb(self) -> Self {
        self & (Self(0u64.wrapping_sub(self.0)))
    }
    pub fn without_lsb(self) -> Self {
        self & Self(self.0.wrapping_sub(1))
    }
    pub fn one(self) -> bool {
        self != Self::EMPTY && self.without_lsb() == Self::EMPTY
    }
    pub fn many(self) -> bool {
        self.without_lsb() != Self::EMPTY
    }
}

/// Iterator over the squares of a square-set.
/// The squares are returned in increasing order.
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

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}
impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}
impl Sub for BitBoard {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}
impl SubAssign for BitBoard {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 &= !rhs.0;
    }
}
impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}
impl Shr<u8> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}
impl Shl<u8> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}
impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LAST_BIT: u64 = 63;
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let char = if self.0 & mask != 0 { '1' } else { '0' };
                write!(f, "{} ", char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{bitboard::BitBoard, types::Square};
    #[test]
    fn to_string() {
        let bbstr = BitBoard(1).to_string();
        let bbstrlen = bbstr.len();
        assert_eq!(bbstrlen, 136);
        assert_eq!(bbstr.starts_with("0 0 0 0 0 0 0 0 \n"), true);
        assert_eq!(bbstr.ends_with("1 0 0 0 0 0 0 0 \n"), true);
    }
    #[test]
    fn empty() {
        let empty = BitBoard::EMPTY;
        assert_eq!(empty, BitBoard(0));
        assert!(!empty.one());
        assert!(!empty.many());
    }
    #[test]
    fn full() {
        let full = BitBoard::FULL;
        assert_eq!(full, BitBoard(18446744073709551615));
        assert!(!full.one());
        assert!(full.many());
    }
    #[test]
    fn north_east_one() {
        let mut bb = BitBoard::from_square(crate::types::Square::A1);
        let mut ne_bb = bb.north_east_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::B2));
        bb = BitBoard::from_square(crate::types::Square::B5);
        ne_bb = bb.north_east_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::C6));
        bb = BitBoard::from_square(crate::types::Square::H4);
        ne_bb = bb.north_east_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
        bb = BitBoard::from_square(crate::types::Square::G8);
        ne_bb = bb.north_east_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
    }
    #[test]
    fn north_west_one() {
        let mut bb = BitBoard::from_square(crate::types::Square::H1);
        let mut ne_bb = bb.north_west_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::G2));
        bb = BitBoard::from_square(crate::types::Square::B5);
        ne_bb = bb.north_west_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::A6));
        bb = BitBoard::from_square(crate::types::Square::A4);
        ne_bb = bb.north_west_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
        bb = BitBoard::from_square(crate::types::Square::G8);
        ne_bb = bb.north_west_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
    }
    #[test]
    fn south_east_one() {
        let mut bb = BitBoard::from_square(crate::types::Square::A2);
        let mut ne_bb = bb.south_east_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::B1));
        bb = BitBoard::from_square(crate::types::Square::B5);
        ne_bb = bb.south_east_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::C4));
        bb = BitBoard::from_square(crate::types::Square::H4);
        ne_bb = bb.south_east_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
        bb = BitBoard::from_square(crate::types::Square::G1);
        ne_bb = bb.south_east_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
    }
    #[test]
    fn south_west_one() {
        let mut bb = BitBoard::from_square(crate::types::Square::H2);
        let mut ne_bb = bb.south_west_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::G1));
        bb = BitBoard::from_square(crate::types::Square::B5);
        ne_bb = bb.south_west_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::A4));
        bb = BitBoard::from_square(crate::types::Square::A4);
        ne_bb = bb.south_west_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
        bb = BitBoard::from_square(crate::types::Square::G1);
        ne_bb = bb.south_west_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
    }
    #[test]
    fn east_one() {
        let mut bb = BitBoard::from_square(crate::types::Square::A2);
        let mut ne_bb = bb.east_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::B2));
        bb = BitBoard::from_square(crate::types::Square::B5);
        ne_bb = bb.east_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::C5));
        bb = BitBoard::from_square(crate::types::Square::H4);
        ne_bb = bb.east_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
    }
    #[test]
    fn west_one() {
        let mut bb = BitBoard::from_square(crate::types::Square::B2);
        let mut ne_bb = bb.west_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::A2));
        bb = BitBoard::from_square(crate::types::Square::B5);
        ne_bb = bb.west_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::A5));
        bb = BitBoard::from_square(crate::types::Square::A4);
        ne_bb = bb.west_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
    }
    #[test]
    fn north_one() {
        let mut bb = BitBoard::from_square(crate::types::Square::B2);
        let mut ne_bb = bb.north_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::B3));
        bb = BitBoard::from_square(crate::types::Square::B5);
        ne_bb = bb.north_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::B6));
        bb = BitBoard::from_square(crate::types::Square::B8);
        ne_bb = bb.north_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
    }
    #[test]
    fn south_one() {
        let mut bb = BitBoard::from_square(crate::types::Square::B2);
        let mut ne_bb = bb.south_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::B1));
        bb = BitBoard::from_square(crate::types::Square::B5);
        ne_bb = bb.south_one();
        assert_eq!(ne_bb, BitBoard::from_square(crate::types::Square::B4));
        bb = BitBoard::from_square(crate::types::Square::B1);
        ne_bb = bb.south_one();
        assert_eq!(ne_bb, BitBoard::EMPTY);
    }
    #[test]
    fn isolate_lsb() {
        let bb = BitBoard::FILE_A;
        let isolated = bb.isolate_lsb();
        assert_eq!(isolated, BitBoard::from_square(crate::types::Square::A1));
    }
    #[test]
    fn without_lsb() {
        let bb = BitBoard::from_square(crate::types::Square::A2)
            | BitBoard::from_square(crate::types::Square::A3);
        let without_lsb = bb.without_lsb();
        assert_eq!(without_lsb, BitBoard::from_square(crate::types::Square::A3));
    }
    #[test]
    fn add_square() {
        let one = Square::E4.as_set();
        assert_ne!(one, BitBoard::EMPTY);
        assert!(one.one());
        assert!(!one.many());

        let two = one.add_square(Square::E5);
        assert_ne!(two, BitBoard::EMPTY);
        assert!(!two.one());
        assert!(two.many());
    }
}
