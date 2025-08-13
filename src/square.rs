use std::{
    //mem::size_of,
    ops::{Index, IndexMut},
    //str::FromStr,
};

use crate::bitboard::BitBoard;
use crate::colour::Colour;
use crate::file::File;
use crate::rank::Rank;

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
//#[allow(clippy::unusual_byte_groupings)]
impl Square {
    pub const fn from_rank_file(rank: Rank, file: File) -> Self {
        let inner = rank as u8 * 8 + file as u8;
        // SAFETY: Rank and File are constrained such that inner is always < 64.
        unsafe { std::mem::transmute(inner) }
    }
    pub const fn new(inner: u8) -> Option<Self> {
        if inner < 64 {
            // SAFETY: inner is less than 64, so it corresponds to a valid enum variant.
            Some(unsafe { std::mem::transmute::<u8, Self>(inner) })
        } else {
            None
        }
    }
    pub const fn new_clamped(inner: u8) -> Self {
        let inner = if inner < 63 { inner } else { 63 };
        let maybe_square = Self::new(inner);
        if let Some(sq) = maybe_square {
            sq
        } else {
            panic!()
        }
    }
    pub const unsafe fn new_unchecked(inner: u8) -> Self {
        debug_assert!(inner < 64);
        unsafe { std::mem::transmute(inner) }
    }
    pub const fn flip_rank(self) -> Self {
        // SAFETY: given the precondition that `self as u8` is less than 64,
        // this operation cannot construct a value >= 64.
        unsafe { std::mem::transmute(self as u8 ^ 0b111_000) }
    }
    pub const fn flip_file(self) -> Self {
        // SAFETY: given the precondition that `self as u8` is less than 64,
        // this operation cannot construct a value >= 64.
        unsafe { std::mem::transmute(self as u8 ^ 0b000_111) }
    }
    pub const fn relative_to(self, side: Colour) -> Self {
        if matches!(side, Colour::White) {
            self
        } else {
            self.flip_rank()
        }
    }
    pub const fn file(self) -> File {
        // SAFETY: `self as u8` is less than 64, and this operation can only
        // decrease the value, so cannot construct a value >= 64.
        unsafe { std::mem::transmute(self as u8 % 8) }
    }
    pub const fn rank(self) -> Rank {
        // SAFETY: `self as u8` is less than 64, and this operation can only
        // decrease the value, so cannot construct a value >= 64.
        unsafe { std::mem::transmute(self as u8 / 8) }
    }
    pub const fn distance(a: Self, b: Self) -> u8 {
        let file_diff = a.file().abs_diff(b.file());
        let rank_diff = a.rank().abs_diff(b.rank());
        if file_diff > rank_diff {
            file_diff
        } else {
            rank_diff
        }
    }
    pub const fn signed_inner(self) -> i8 {
        //#![allow(clippy::cast_possible_wrap)]
        self as i8
    }
    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn as_bb(self) -> BitBoard {
        1u64 << self.index()
    }
}
impl<T> Index<Square> for [T; 64] {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked(index as usize) }
    }
}
impl<T> IndexMut<Square> for [T; 64] {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}
