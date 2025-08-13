use crate::consts::*;
use std::ops::{Index, IndexMut};

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

#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn abs_diff() {
        assert_eq!(File::A.abs_diff(File::H), 7);
        assert_eq!(File::H.abs_diff(File::A), 7);
        assert_eq!(File::A.abs_diff(File::A), 0);
        assert_eq!(File::H.abs_diff(File::H), 0);
    }
    #[test]
    fn from_index() {
        assert_eq!(File::from_index(0), Some(File::A));
        assert_eq!(File::from_index(1), Some(File::B));
        assert_eq!(File::from_index(2), Some(File::C));
        assert_eq!(File::from_index(3), Some(File::D));
        assert_eq!(File::from_index(4), Some(File::E));
        assert_eq!(File::from_index(5), Some(File::F));
        assert_eq!(File::from_index(6), Some(File::G));
        assert_eq!(File::from_index(7), Some(File::H));
        assert_eq!(File::from_index(8), None);
    }
    #[test]
    fn add() {
        assert_eq!(File::A.add(0), Some(File::A));
        assert_eq!(File::A.add(1), Some(File::B));
        assert_eq!(File::H.add(0), Some(File::H));
        assert_eq!(File::H.add(1), None);
        assert_eq!(File::A.add(7), Some(File::H));
        assert_eq!(File::H.add(7), None);
    }
    #[test]
    fn sub() {
        assert_eq!(File::A.sub(0), Some(File::A));
        assert_eq!(File::B.sub(1), Some(File::A));
        assert_eq!(File::H.sub(0), Some(File::H));
        assert_eq!(File::H.sub(1), Some(File::G));
        assert_eq!(File::A.sub(1), None);
        assert_eq!(File::H.sub(7), Some(File::A));
        assert_eq!(File::A.sub(7), None);
    }
    #[test]
    fn all() {
        let mut files = File::all();
        assert_eq!(files.next(), Some(File::A));
        assert_eq!(files.next(), Some(File::B));
        assert_eq!(files.next(), Some(File::C));
        assert_eq!(files.next(), Some(File::D));
        assert_eq!(files.next(), Some(File::E));
        assert_eq!(files.next(), Some(File::F));
        assert_eq!(files.next(), Some(File::G));
        assert_eq!(files.next(), Some(File::H));
        assert_eq!(files.next(), None);
    }
    #[test]
    fn with() {
        assert_eq!(File::A.with(Rank::One), Square::A1);
        assert_eq!(File::B.with(Rank::Two), Square::B2);
        assert_eq!(File::C.with(Rank::Three), Square::C3);
        assert_eq!(File::D.with(Rank::Four), Square::D4);
        assert_eq!(File::E.with(Rank::Five), Square::E5);
        assert_eq!(File::F.with(Rank::Six), Square::F6);
        assert_eq!(File::G.with(Rank::Seven), Square::G7);
        assert_eq!(File::H.with(Rank::Eight), Square::H8);
    }
}
