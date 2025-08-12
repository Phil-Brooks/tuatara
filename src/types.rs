use std::{
    //fmt::{self, Display},
    //mem::size_of,
    ops::{Index},//, IndexMut},
    //str::FromStr,
};

use crate::bitboard::BitBoard;

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
const _SQUARE_ASSERT: () = assert!(size_of::<Square>() == size_of::<Option<Square>>());
impl<T> Index<Square> for [T; 64] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked(index as usize) }
    }
}

//#[allow(clippy::unusual_byte_groupings)]
impl Square {
    pub const fn index(self) -> usize {
        self as usize
    }
    /// SAFETY: you may only call this function with value of `inner` less than 64.
    pub const unsafe fn new_unchecked(inner: u8) -> Self {
        debug_assert!(inner < 64);
        unsafe { std::mem::transmute(inner) }
    }
    pub const fn as_set(self) -> BitBoard {
        BitBoard(1u64 << self.index())
    }
}