pub mod attacks;
pub mod bitboard;
pub mod board;
pub mod col;
pub mod consts;
pub mod file;
pub mod piece;
pub mod piecetype;
pub mod rank;
pub mod square;

use crate::consts::*;

fn main() {
    let bd = Board::new();
    println!("Initial board state:\n{:#?}", bd);
}
