pub mod bitboard;
pub mod colour;
pub mod consts;
pub mod file;
pub mod piece;
pub mod piecetype;
pub mod rank;
pub mod square;

use crate::consts::BitBoard;

fn main() {
    let mut bb: BitBoard = 1;
    println!("bb1:\n{}", bitboard::to_string(bb));
    bb = 2;
    let bb2str = bitboard::to_string(bb);
    println!("bb2:\n{}", bitboard::to_string(bb));
    println!("bb2str:\n{}", bb2str);
    bb = 3;
    println!("bb3:\n{}", bitboard::to_string(bb));
}
