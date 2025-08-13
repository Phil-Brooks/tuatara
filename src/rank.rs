use crate::consts::*;
use std::ops::{Index, IndexMut};

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

#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn abs_diff() {
        assert_eq!(Rank::One.abs_diff(Rank::Three), 2);
        assert_eq!(Rank::Eight.abs_diff(Rank::Five), 3);
        assert_eq!(Rank::Four.abs_diff(Rank::Four), 0);
    }
    #[test]
    fn from_index() {
        assert_eq!(Rank::from_index(0), Some(Rank::One));
        assert_eq!(Rank::from_index(7), Some(Rank::Eight));
        assert_eq!(Rank::from_index(8), None);
    }
    #[test]
    fn add() {
        assert_eq!(Rank::One.add(2), Some(Rank::Three));
        assert_eq!(Rank::Eight.add(1), None);
    }
    #[test]
    fn sub() {
        assert_eq!(Rank::Three.sub(2), Some(Rank::One));
        assert_eq!(Rank::One.sub(1), None);
    }
    #[test]
    fn all() {
        let ranks: Vec<Rank> = Rank::all().collect();
        assert_eq!(
            ranks,
            vec![
                Rank::One,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight
            ]
        );
    }
}
