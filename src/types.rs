use std::{
    //fmt::{self, Display},
    //mem::size_of,
    ops::{Index, IndexMut},
    //str::FromStr,
};

use crate::bitboard::BitBoard;

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

const _FILE_ASSERT: () = assert!(size_of::<File>() == size_of::<Option<File>>());

impl File {
    pub const fn abs_diff(self, other: Self) -> u8 {
        (self as u8).abs_diff(other as u8)
    }
    pub const fn from_index(index: u8) -> Option<Self> {
        if index < 8 {
            // SAFETY: inner is less than 8, so it corresponds to a valid enum variant.
            Some(unsafe { std::mem::transmute::<u8, Self>(index) })
        } else {
            None
        }
    }
    pub const fn add(self, diff: u8) -> Option<Self> {
        Self::from_index(self as u8 + diff)
    }
    pub const fn sub(self, diff: u8) -> Option<Self> {
        #![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        Self::from_index((self as i8 - diff as i8) as u8)
    }
    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        // SAFETY: all values are within `0..64`.
        (0..8u8).map(|i| unsafe { std::mem::transmute(i) })
    }
    pub const fn with(self, rank: Rank) -> Square {
        Square::from_rank_file(rank, self)
    }
}

impl<T> Index<File> for [T; 8] {
    type Output = T;

    fn index(&self, index: File) -> &Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked(index as usize) }
    }
}

impl<T> IndexMut<File> for [T; 8] {
    fn index_mut(&mut self, index: File) -> &mut Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked_mut(index as usize) }
    }
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

const _RANK_ASSERT: () = assert!(size_of::<Rank>() == size_of::<Option<Rank>>());

impl Rank {
    pub const fn abs_diff(self, other: Self) -> u8 {
        (self as u8).abs_diff(other as u8)
    }
    pub const fn from_index(index: u8) -> Option<Self> {
        if index < 8 {
            // SAFETY: inner is less than 8, so it corresponds to a valid enum variant.
            Some(unsafe { std::mem::transmute::<u8, Self>(index) })
        } else {
            None
        }
    }
    pub const fn add(self, diff: u8) -> Option<Self> {
        Self::from_index(self as u8 + diff)
    }
    pub const fn sub(self, diff: u8) -> Option<Self> {
        #![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        Self::from_index((self as i8 - diff as i8) as u8)
    }
    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        // SAFETY: all values are within `0..8`.
        (0..8u8).map(|i| unsafe { std::mem::transmute(i) })
    }
    pub const fn with(self, file: File) -> Square {
        Square::from_rank_file(self, file)
    }
}

impl<T> Index<Rank> for [T; 8] {
    type Output = T;

    fn index(&self, index: Rank) -> &Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked(index as usize) }
    }
}

impl<T> IndexMut<Rank> for [T; 8] {
    fn index_mut(&mut self, index: Rank) -> &mut Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked_mut(index as usize) }
    }
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
const _SQUARE_ASSERT: () = assert!(size_of::<Square>() == size_of::<Option<Square>>());

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

static SQUARE_NAMES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

//#[allow(clippy::unusual_byte_groupings)]
impl Square {
    pub const fn from_rank_file(rank: Rank, file: File) -> Self {
        let inner = rank as u8 * 8 + file as u8;
        // SAFETY: Rank and File are constrained such that inner is always < 64.
        unsafe { std::mem::transmute(inner) }
    }

    pub const fn index(self) -> usize {
        self as usize
    }
    /// SAFETY: you may only call this function with value of `inner` less than 64.
    pub const unsafe fn new_unchecked(inner: u8) -> Self {
        debug_assert!(inner < 64);
        unsafe { std::mem::transmute(inner) }
    }
    pub const fn as_bb(self) -> BitBoard {
        BitBoard(1u64 << self.index())
    }
}
