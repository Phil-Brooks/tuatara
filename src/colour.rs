use crate::consts::Colour;
use std::fmt::Display;
use std::ops::{Index, IndexMut, Not};

impl Colour {
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
impl Not for Colour {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.flip()
    }
}
impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::White => write!(f, "w"),
            Self::Black => write!(f, "b"),
        }
    }
}
impl<T> Index<Colour> for [T; 2] {
    type Output = T;

    fn index(&self, index: Colour) -> &Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked(index as usize) }
    }
}
impl<T> IndexMut<Colour> for [T; 2] {
    fn index_mut(&mut self, index: Colour) -> &mut Self::Output {
        // SAFETY: the legal values for this type are all in bounds.
        unsafe { self.get_unchecked_mut(index as usize) }
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn new() {
        assert_eq!(Colour::new(true), Colour::Black);
        assert_eq!(Colour::new(false), Colour::White);
    }
    #[test]
    fn flip() {
        assert_eq!(Colour::White.flip(), Colour::Black);
        assert_eq!(Colour::Black.flip(), Colour::White);
    }
    #[test]
    fn index() {
        assert_eq!(Colour::White.index(), 0);
        assert_eq!(Colour::Black.index(), 1);
    }
    #[test]
    fn inner() {
        assert_eq!(Colour::White.inner(), 0);
        assert_eq!(Colour::Black.inner(), 1);
    }
    #[test]
    fn all() {
        let mut iter = Colour::all();
        assert_eq!(iter.next(), Some(Colour::White));
        assert_eq!(iter.next(), Some(Colour::Black));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn display() {
        assert_eq!(format!("{}", Colour::White), "w");
        assert_eq!(format!("{}", Colour::Black), "b");
    }
    #[test]
    fn not() {
        assert_eq!(!Colour::White, Colour::Black);
        assert_eq!(!Colour::Black, Colour::White);
    }
    #[test]
    fn index_trait() {
        let arr: [u8; 2] = [1, 2];
        assert_eq!(arr[Colour::White], 1);
        assert_eq!(arr[Colour::Black], 2);
    }
}
