use crate::consts::Col;
use std::fmt::Display;
use std::ops::{Index, IndexMut, Not};

impl Col {
    pub const fn new(v: bool) -> Self {
        if v { Self::Black } else { Self::White }
    }
    pub const fn flip(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
    pub const fn index(self) -> usize {
        self as usize
    }
    pub const fn inner(self) -> u8 {
        self as u8
    }
    pub fn all() -> impl DoubleEndedIterator<Item = Self> {
        [Self::White, Self::Black].into_iter()
    }
}
impl Not for Col {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.flip()
    }
}
impl Display for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::White => write!(f, "w"),
            Self::Black => write!(f, "b"),
        }
    }
}
impl<T> Index<Col> for [T; 2] {
    type Output = T;

    fn index(&self, index: Col) -> &Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked(index as usize) }
    }
}
impl<T> IndexMut<Col> for [T; 2] {
    fn index_mut(&mut self, index: Col) -> &mut Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn new() {
        assert_eq!(Col::new(true), Col::Black);
        assert_eq!(Col::new(false), Col::White);
    }
    #[test]
    fn flip() {
        assert_eq!(Col::White.flip(), Col::Black);
        assert_eq!(Col::Black.flip(), Col::White);
    }
    #[test]
    fn index() {
        assert_eq!(Col::White.index(), 0);
        assert_eq!(Col::Black.index(), 1);
    }
    #[test]
    fn inner() {
        assert_eq!(Col::White.inner(), 0);
        assert_eq!(Col::Black.inner(), 1);
    }
    #[test]
    fn all() {
        let mut iter = Col::all();
        assert_eq!(iter.next(), Some(Col::White));
        assert_eq!(iter.next(), Some(Col::Black));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn display() {
        assert_eq!(format!("{}", Col::White), "w");
        assert_eq!(format!("{}", Col::Black), "b");
    }
    #[test]
    fn not() {
        assert_eq!(!Col::White, Col::Black);
        assert_eq!(!Col::Black, Col::White);
    }
    #[test]
    fn index_trait() {
        let arr: [u8; 2] = [1, 2];
        assert_eq!(arr[Col::White], 1);
        assert_eq!(arr[Col::Black], 2);
    }
}
