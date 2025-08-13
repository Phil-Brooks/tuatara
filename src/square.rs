use std::{
    fmt::{self, Display},
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
        if offset > self as u8 {
            None
        } else {
            let res = self as u8 - offset;
            Self::new(res)
        }
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

#[cfg(test)]
mod tests {
    use crate::consts::*;
    use std::str::FromStr;
    #[test]
    fn to_string() {
        let bbstr = Square::B3.to_string();
        let bbstrlen = bbstr.len();
        assert_eq!(bbstrlen, 2);
        assert_eq!(bbstr, "b3");
    }
    #[test]
    fn from_str() {
        let square = Square::from_str("b3").unwrap();
        assert_eq!(square, Square::B3);
        assert!(Square::from_str("invalid").is_err());
    }
    #[test]
    fn from_rank_file() {
        let square = Square::from_rank_file(Rank::Three, File::B);
        assert_eq!(square, Square::B3);
    }
    #[test]
    fn flip_rank() {
        let square = Square::A1.flip_rank();
        assert_eq!(square, Square::A8);
        let square = Square::H8.flip_rank();
        assert_eq!(square, Square::H1);
    }
    #[test]
    fn flip_file() {
        let square = Square::A1.flip_file();
        assert_eq!(square, Square::H1);
        let square = Square::H8.flip_file();
        assert_eq!(square, Square::A8);
    }
    #[test]
    fn relative_to() {
        let square = Square::A1.relative_to(Colour::White);
        assert_eq!(square, Square::A1);
        let square = Square::A1.relative_to(Colour::Black);
        assert_eq!(square, Square::A8);
    }
    #[test]
    fn file() {
        let square = Square::B3;
        let file = square.file();
        assert_eq!(file, File::B);
    }
    #[test]
    fn rank() {
        let square = Square::B3;
        let rank = square.rank();
        assert_eq!(rank, Rank::Three);
    }
    #[test]
    fn distance() {
        let square1 = Square::A1;
        let square2 = Square::B2;
        let distance = Square::distance(square1, square2);
        assert_eq!(distance, 1);
        let square3 = Square::C3;
        let distance = Square::distance(square1, square3);
        assert_eq!(distance, 2);
    }
    #[test]
    fn index() {
        let square = Square::B3;
        let index = square.index();
        assert_eq!(index, 17);
        let square_from_index = Square::new(index as u8).unwrap();
        assert_eq!(square, square_from_index);
    }
    #[test]
    fn inner() {
        let square = Square::B3;
        let inner = square.inner();
        assert_eq!(inner, 17);
        let square_from_inner = Square::new(inner).unwrap();
        assert_eq!(square, square_from_inner);
    }
    #[test]
    fn add() {
        let square = Square::B3;
        let new_square = square.add(2).unwrap();
        assert_eq!(new_square, Square::D3);
        let out_of_bounds_square = square.add(100);
        assert!(out_of_bounds_square.is_none());
    }
    #[test]
    fn sub() {
        let square = Square::B3;
        let new_square = square.sub(2).unwrap();
        assert_eq!(new_square, Square::H2);
        let out_of_bounds_square = square.sub(100);
        assert!(out_of_bounds_square.is_none());
    }
    #[test]
    fn as_bb() {
        let square = Square::B3;
        let bb = square.as_bb();
        assert_eq!(bb, 1u64 << 17);
    }
    #[test]
    fn pawn_push() {
        let square = Square::B3;
        let pushed_square = square.pawn_push(Colour::White).unwrap();
        assert_eq!(pushed_square, Square::B4);
        let pushed_square_black = square.pawn_push(Colour::Black).unwrap();
        assert_eq!(pushed_square_black, Square::B2);
    }
    #[test]
    fn pawn_right() {
        let square = Square::B3;
        let right_square = square.pawn_right(Colour::White).unwrap();
        assert_eq!(right_square, Square::C4);
        let right_square_black = square.pawn_right(Colour::Black).unwrap();
        assert_eq!(right_square_black, Square::C2);
    }
    #[test]
    fn pawn_left() {
        let square = Square::B3;
        let left_square = square.pawn_left(Colour::White).unwrap();
        assert_eq!(left_square, Square::A4);
        let left_square_black = square.pawn_left(Colour::Black).unwrap();
        assert_eq!(left_square_black, Square::A2);
    }
    #[test]
    fn le_ge_lt_gt() {
        let square1 = Square::B3;
        let square2 = Square::C4;
        assert!(square1.le(square2));
        assert!(square1.lt(square2));
        assert!(!square1.ge(square2));
        assert!(!square1.gt(square2));

        let square3 = Square::B3;
        assert!(square3.le(square1));
        assert!(!square3.lt(square1));
        assert!(square3.ge(square1));
        assert!(!square3.gt(square1));
    }
    #[test]
    fn all_squares() {
        let squares: Vec<Square> = Square::all().collect();
        assert_eq!(squares.len(), 64);
        for (i, square) in squares.iter().enumerate() {
            assert_eq!(*square, Square::new(i as u8).unwrap());
        }
    }
    #[test]
    fn name() {
        let square = Square::B3;
        let name = square.name();
        assert_eq!(name, "b3");
        let square_from_name = Square::from_str(name).unwrap();
        assert_eq!(square, square_from_name);
    }
}
