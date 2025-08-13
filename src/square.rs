use std::{
    fmt::{self, Display},
    //mem::size_of,
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::consts::*;

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
        self as i8
    }
    pub const fn index(self) -> usize {
        self as usize
    }
    pub const fn inner(self) -> u8 {
        self as u8
    }
    pub const fn add(self, offset: u8) -> Option<Self> {
        let res = self as u8 + offset;
        Self::new(res)
    }
    pub const fn saturating_add(self, offset: u8) -> Self {
        let res = self as u8 + offset;
        let inner = if res < 63 { res } else { 63 };
        let maybe_square = Self::new(inner);
        if let Some(sq) = maybe_square {
            sq
        } else {
            panic!()
        }
    }
    pub const unsafe fn add_unchecked(self, offset: u8) -> Self {
        let res = self as u8 + offset;
        unsafe { Self::new_unchecked(res) }
    }
    pub const unsafe fn sub_unchecked(self, offset: u8) -> Self {
        let res = self as u8 - offset;
        unsafe { Self::new_unchecked(res) }
    }
    pub const fn sub(self, offset: u8) -> Option<Self> {
        let res = self as u8 - offset;
        Self::new(res)
    }
    pub const fn as_bb(self) -> BitBoard {
        1u64 << self.index()
    }
    pub fn pawn_push(self, side: Colour) -> Option<Self> {
        if side == Colour::White {
            self.add(8)
        } else {
            self.sub(8)
        }
    }
    pub fn pawn_right(self, side: Colour) -> Option<Self> {
        if side == Colour::White {
            self.add(9)
        } else {
            self.sub(7)
        }
    }
    pub fn pawn_left(self, side: Colour) -> Option<Self> {
        if side == Colour::White {
            self.add(7)
        } else {
            self.sub(9)
        }
    }
    pub const fn le(self, other: Self) -> bool {
        self as u8 <= other as u8
    }
    pub const fn ge(self, other: Self) -> bool {
        self as u8 >= other as u8
    }
    pub const fn lt(self, other: Self) -> bool {
        (self as u8) < other as u8
    }
    pub const fn gt(self, other: Self) -> bool {
        self as u8 > other as u8
    }
    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        (0..64u8).map(|i| unsafe { std::mem::transmute(i) })
    }
    pub fn name(self) -> &'static str {
        SQUARE_NAMES[self]
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
impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SQUARE_NAMES[*self])
    }
}
impl FromStr for Square {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SQUARE_NAMES
            .iter()
            .position(|&name| name == s)
            .and_then(|index| -> Option<u8> { index.try_into().ok() })
            .and_then(Self::new)
            .ok_or("Invalid square name")
    }
}
impl From<Square> for u16 {
    fn from(square: Square) -> Self {
        square as Self
    }
}
