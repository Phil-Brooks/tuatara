pub mod bitboard;
pub mod colour;
pub mod consts;
pub mod file;
pub mod rank;
pub mod square;

use crate::bitboard::BitBoard;

fn main() {
    let mut bb = BitBoard(1);
    println!("bb1:\n{}", bb);
    bb = BitBoard(2);
    let bb2str = bb.to_string();
    println!("bb2:\n{}", bb);
    println!("bb2str:\n{}", bb2str);
    bb = BitBoard(3);
    println!("bb3:\n{}", bb);
}
