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
    let mut bb: BitBoard = bd.by_piece[PieceType::Pawn.index()];
    println!("bb1:\n{}", bitboard::to_string(bb));
    bb = bd.by_col[Col::Black.index()];
    let bb2str = bitboard::to_string(bb);
    println!("bb2:\n{}", bitboard::to_string(bb));
    println!("bb2str:\n{}", bb2str);
    bb = bd.occupied;
    println!("bb3:\n{}", bitboard::to_string(bb));
}
