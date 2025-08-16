use std::fmt;

use crate::attacks;
use crate::bitboard;
use crate::consts::*;

impl Board {
    pub const fn new() -> Self {
        Board {
            by_piece: [
                0x00ff_0000_0000_ff00,
                0x4200_0000_0000_0042,
                0x2400_0000_0000_0024,
                0x8100_0000_0000_0081,
                0x0800_0000_0000_0008,
                0x1000_0000_0000_0010,
            ],
            by_col: [0xffff, 0xffff_0000_0000_0000],
            occupied: 0xffff_0000_0000_ffff,
        }
    }
    pub const fn empty() -> Self {
        Board {
            by_piece: [BB_EMPTY; 6],
            by_col: [BB_EMPTY; 2],
            occupied: BB_EMPTY,
        }
    }
    pub const fn try_from_bitboards(
        by_piece: [BitBoard; 6],
        by_col: [BitBoard; 2],
    ) -> Result<Board, &'static str> {
        let mut occupied = BB_EMPTY;
        let mut i = 0;
        while i < by_piece.len() {
            occupied |= by_piece[i];
            i += 1;
        }

        let mut sum = 0;
        let mut j = 0;
        while j < by_piece.len() {
            sum += by_piece[j].count_ones();
            j += 1;
        }
        if occupied.count_ones() != sum {
            return Err("roles overlap");
        }

        if by_col[Col::Black.index()] & by_col[Col::White.index()] != BB_EMPTY {
            return Err("colours overlap");
        }

        if occupied != by_col[Col::Black.index()] | by_col[Col::White.index()] {
            return Err("roles and colours are mismatched");
        }

        Ok(Board {
            by_piece,
            by_col,
            occupied,
        })
    }
    pub const fn into_bitboards(self) -> ([BitBoard; 6], [BitBoard; 2]) {
        (self.by_piece, self.by_col)
    }
    pub const fn occupied(&self) -> BitBoard {
        self.occupied
    }
    pub const fn pawns(&self) -> BitBoard {
        self.by_piece[PieceType::Pawn.index()]
    }
    pub const fn knights(&self) -> BitBoard {
        self.by_piece[PieceType::Knight.index()]
    }
    pub const fn bishops(&self) -> BitBoard {
        self.by_piece[PieceType::Bishop.index()]
    }
    pub const fn rooks(&self) -> BitBoard {
        self.by_piece[PieceType::Rook.index()]
    }
    pub const fn queens(&self) -> BitBoard {
        self.by_piece[PieceType::Queen.index()]
    }
    pub const fn kings(&self) -> BitBoard {
        self.by_piece[PieceType::King.index()]
    }
    pub const fn white(&self) -> BitBoard {
        self.by_col[Col::White.index()]
    }
    pub const fn black(&self) -> BitBoard {
        self.by_col[Col::Black.index()]
    }
    pub const fn sliders(&self) -> BitBoard {
        self.rooks() ^ self.bishops() ^ self.queens()
    }
    pub const fn steppers(&self) -> BitBoard {
        self.knights() ^ self.pawns() ^ self.kings()
    }
    pub const fn rooks_and_queens(&self) -> BitBoard {
        self.rooks() ^ self.queens()
    }
    pub const fn bishops_and_queens(&self) -> BitBoard {
        self.bishops() ^ self.queens()
    }
    pub const fn king_of(&self, colour: Col) -> Square {
        bitboard::first(self.by_piece[PieceType::King.index()] & self.by_col[colour.index()])
    }
    pub fn col_at(&self, square: Square) -> Option<Col> {
        let mask = bitboard::from_square(square);
        if self.by_col[Col::White.index()] & mask != BB_EMPTY {
            Some(Col::White)
        } else if self.by_col[Col::Black.index()] & mask != BB_EMPTY {
            Some(Col::Black)
        } else {
            None
        }
    }
    pub fn piecetype_at(&self, square: Square) -> Option<PieceType> {
        let mask = bitboard::from_square(square);
        let mut i = 0;
        while i < self.by_piece.len() {
            let bb = self.by_piece[i];
            if bb & mask != BB_EMPTY {
                return Some(unsafe { PieceType::from_index_unchecked(i as u8) });
            }
            i += 1;
        }
        None
    }
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let piecetype = self.piecetype_at(square);
        let col = self.col_at(square);
        if piecetype.is_none() || col.is_none() {
            return None;
        }
        Some(Piece::from_piecetype_and_col(
            piecetype.unwrap(),
            col.unwrap(),
        ))
    }
    pub fn remove_piece_at(&mut self, sq: Square) -> Option<Piece> {
        let piece = self.piece_at(sq);
        if piece.is_none() {
            return None;
        }
        let piece = piece.unwrap();
        let mask = bitboard::from_square(sq);
        self.by_piece[piece.piecetype().index()] &= !mask;
        self.by_col[piece.col().index()] &= !mask;
        self.occupied &= !mask;
        Some(piece)
    }
    pub fn discard_piece_at(&mut self, sq: Square) {
        let piece = self.piece_at(sq);
        if piece.is_none() {
            return;
        }
        let piece = piece.unwrap();
        let mask = bitboard::from_square(sq);
        self.by_piece[piece.piecetype().index()] &= !mask;
        self.by_col[piece.col().index()] &= !mask;
        self.occupied &= !mask;
    }
    pub fn set_piece_at(&mut self, sq: Square, piece: Piece) {
        self.discard_piece_at(sq);
        let mask = bitboard::from_square(sq);
        self.by_piece[piece.piecetype().index()] |= mask;
        self.by_col[piece.col().index()] |= mask;
        self.occupied |= mask;
    }
    pub fn set_new_piece_at(&mut self, sq: Square, piece: Piece) {
        if self.piece_at(sq).is_some() {
            panic!("Cannot set a piece at a square that is already occupied");
        }
        let mask = bitboard::from_square(sq);
        self.by_piece[piece.piecetype().index()] |= mask;
        self.by_col[piece.col().index()] |= mask;
        self.occupied |= mask;
    }
    pub const fn by_col(&self, col: Col) -> BitBoard {
        self.by_col[col.index()]
    }
    pub const fn by_piecetype(&self, piecetype: PieceType) -> BitBoard {
        self.by_piece[piecetype.index()]
    }
    pub const fn by_piece(&self, piece: Piece) -> BitBoard {
        self.by_piece[piece.piecetype().index()] & self.by_col[piece.col().index()]
    }
    pub fn attacks_from(&self, sq: Square) -> BitBoard {
        let piece = self.piece_at(sq);
        if piece.is_none() {
            return BB_EMPTY;
        }
        let piece = piece.unwrap();
        attacks::attacks(sq, piece, self.occupied)
    }
    pub fn atacks_to(&self, sq: Square, attacker: Col, occupied: BitBoard) -> BitBoard {
        self.by_col(attacker)
            & ((attacks::rook_attacks(sq, occupied) & self.rooks_and_queens())
                | (attacks::bishop_attacks(sq, occupied) & self.bishops_and_queens())
                | (attacks::knight_attacks(sq) & self.by_piecetype(PieceType::Knight))
                | (attacks::king_attacks(sq) & self.by_piecetype(PieceType::King))
                | (attacks::pawn_attacks(attacker.flip(), sq) & self.by_piecetype(PieceType::Pawn)))
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in Rank::all().rev() {
            for file in File::all() {
                let square = Square::from_rank_file(rank, file);
                let piece = self.piece_at(square);
                if let Some(piece) = piece {
                    write!(f, "{} ", piece)?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::attacks;
    use crate::bitboard;
    use crate::consts::*;
    #[test]
    fn new() {
        let board = Board::new();
        assert_eq!(
            board.by_piece[PieceType::Pawn.index()],
            0x00ff_0000_0000_ff00
        );
        assert_eq!(board.by_col[Col::White.index()], 0xffff);
        assert_eq!(board.occupied, 0xffff_0000_0000_ffff);
    }
    #[test]
    fn empty() {
        let board = Board::empty();
        assert_eq!(board.by_piece, [BB_EMPTY; 6]);
        assert_eq!(board.by_col, [BB_EMPTY; 2]);
        assert_eq!(board.occupied, BB_EMPTY);
    }
    #[test]
    fn try_from_bitboards() {
        let mut by_piece = [
            0x00ff_0000_0000_ff00,
            0x4200_0000_0000_0042,
            0x2400_0000_0000_0024,
            0x8100_0000_0000_0081,
            0x0800_0000_0000_0008,
            0x1000_0000_0000_0010,
        ];
        let mut by_col = [BB_EMPTY, BB_EMPTY];
        let mut board = Board::try_from_bitboards(by_piece, by_col);
        let err = board.unwrap_err();
        assert_eq!(err, "roles and colours are mismatched");
        by_col = [BB_FULL, BB_RANK_1];
        board = Board::try_from_bitboards(by_piece, by_col);
        let err = board.unwrap_err();
        assert_eq!(err, "colours overlap");
        by_piece[PieceType::Pawn.index()] = BB_FULL;
        board = Board::try_from_bitboards(by_piece, by_col);
        let err = board.unwrap_err();
        assert_eq!(err, "roles overlap");
        let nbd = Board::new();
        by_col = nbd.by_col;
        by_piece = nbd.by_piece;
        board = Board::try_from_bitboards(by_piece, by_col);
        assert!(board.is_ok());
    }
    #[test]
    fn into_bitboards() {
        let nbd = Board::new();
        let (by_piece, by_col) = nbd.clone().into_bitboards();
        assert_eq!(by_piece[PieceType::Pawn.index()], 0x00ff_0000_0000_ff00);
        assert_eq!(by_col[Col::White.index()], 0xffff);
        assert_eq!(nbd.occupied, 0xffff_0000_0000_ffff);
    }
    #[test]
    fn occupied() {
        let nbd = Board::new();
        assert_eq!(nbd.occupied(), 0xffff_0000_0000_ffff);
    }
    #[test]
    fn pawns() {
        let nbd = Board::new();
        assert_eq!(nbd.pawns(), 0x00ff_0000_0000_ff00);
    }
    #[test]
    fn knights() {
        let nbd = Board::new();
        assert_eq!(nbd.knights(), 0x4200_0000_0000_0042);
    }
    #[test]
    fn bishops() {
        let nbd = Board::new();
        assert_eq!(nbd.bishops(), 0x2400_0000_0000_0024);
    }
    #[test]
    fn rooks() {
        let nbd = Board::new();
        assert_eq!(nbd.rooks(), 0x8100_0000_0000_0081);
    }
    #[test]
    fn queens() {
        let nbd = Board::new();
        assert_eq!(nbd.queens(), 0x0800_0000_0000_0008);
    }
    #[test]
    fn kings() {
        let nbd = Board::new();
        assert_eq!(nbd.kings(), 0x1000_0000_0000_0010);
    }
    #[test]
    fn white() {
        let nbd = Board::new();
        assert_eq!(nbd.white(), 0xffff);
    }
    #[test]
    fn black() {
        let nbd = Board::new();
        assert_eq!(nbd.black(), 0xffff_0000_0000_0000);
    }
    #[test]
    fn sliders() {
        let nbd = Board::new();
        assert_eq!(
            nbd.sliders(),
            0x8100_0000_0000_0081 ^ 0x2400_0000_0000_0024 ^ 0x0800_0000_0000_0008
        );
    }
    #[test]
    fn steppers() {
        let nbd = Board::new();
        assert_eq!(
            nbd.steppers(),
            0x4200_0000_0000_0042 ^ 0x00ff_0000_0000_ff00 ^ 0x1000_0000_0000_0010
        );
    }
    #[test]
    fn rooks_and_queens() {
        let nbd = Board::new();
        assert_eq!(
            nbd.rooks_and_queens(),
            0x8100_0000_0000_0081 ^ 0x0800_0000_0000_0008
        );
    }
    #[test]
    fn bishops_and_queens() {
        let nbd = Board::new();
        assert_eq!(
            nbd.bishops_and_queens(),
            0x2400_0000_0000_0024 ^ 0x0800_0000_0000_0008
        );
    }
    #[test]
    fn king_of() {
        let nbd = Board::new();
        assert_eq!(nbd.king_of(Col::White), Square::E1);
        assert_eq!(nbd.king_of(Col::Black), Square::E8);
    }
    #[test]
    fn col_at() {
        let nbd = Board::new();
        assert_eq!(nbd.col_at(Square::A1), Some(Col::White));
        assert_eq!(nbd.col_at(Square::E8), Some(Col::Black));
        assert_eq!(nbd.col_at(Square::D4), None);
    }
    #[test]
    fn piecetype_at() {
        let nbd = Board::new();
        assert_eq!(nbd.piecetype_at(Square::A1), Some(PieceType::Rook));
        assert_eq!(nbd.piecetype_at(Square::E8), Some(PieceType::King));
        assert_eq!(nbd.piecetype_at(Square::D2), Some(PieceType::Pawn));
        assert_eq!(nbd.piecetype_at(Square::H8), Some(PieceType::Rook));
        assert_eq!(nbd.piecetype_at(Square::A5), None);
    }
    #[test]
    fn piece_at() {
        let nbd = Board::new();
        assert_eq!(nbd.piece_at(Square::A1), Some(Piece::WR));
        assert_eq!(nbd.piece_at(Square::E8), Some(Piece::BK));
        assert_eq!(nbd.piece_at(Square::D2), Some(Piece::WP));
        assert_eq!(nbd.piece_at(Square::H8), Some(Piece::BR));
        assert_eq!(nbd.piece_at(Square::A5), None);
    }
    #[test]
    fn remove_piece_at() {
        let mut nbd = Board::new();
        let piece = nbd.remove_piece_at(Square::A1);
        assert_eq!(piece, Some(Piece::WR));
        assert_eq!(nbd.piece_at(Square::A1), None);
        assert_eq!(
            nbd.by_piece[PieceType::Rook.index()] & bitboard::from_square(Square::A1),
            BB_EMPTY
        );
        assert_eq!(
            nbd.by_col[Col::White.index()] & bitboard::from_square(Square::A1),
            BB_EMPTY
        );
        assert_eq!(nbd.occupied & bitboard::from_square(Square::A1), BB_EMPTY);
    }
    #[test]
    fn discard_piece_at() {
        let mut nbd = Board::new();
        nbd.discard_piece_at(Square::A1);
        assert_eq!(nbd.piece_at(Square::A1), None);
        assert_eq!(
            nbd.by_piece[PieceType::Rook.index()] & bitboard::from_square(Square::A1),
            BB_EMPTY
        );
        assert_eq!(
            nbd.by_col[Col::White.index()] & bitboard::from_square(Square::A1),
            BB_EMPTY
        );
        assert_eq!(nbd.occupied & bitboard::from_square(Square::A1), BB_EMPTY);
    }
    #[test]
    fn set_piece_at() {
        let mut nbd = Board::empty();
        nbd.set_piece_at(Square::A1, Piece::WR);
        assert_eq!(nbd.piece_at(Square::A1), Some(Piece::WR));
        assert_eq!(
            nbd.by_piece[PieceType::Rook.index()] & bitboard::from_square(Square::A1),
            bitboard::from_square(Square::A1)
        );
        assert_eq!(
            nbd.by_col[Col::White.index()] & bitboard::from_square(Square::A1),
            bitboard::from_square(Square::A1)
        );
        assert_eq!(
            nbd.occupied & bitboard::from_square(Square::A1),
            bitboard::from_square(Square::A1)
        );
    }
    #[test]
    fn set_new_piece_at() {
        let mut nbd = Board::empty();
        nbd.set_new_piece_at(Square::A1, Piece::WR);
        assert_eq!(nbd.piece_at(Square::A1), Some(Piece::WR));
        assert_eq!(
            nbd.by_piece[PieceType::Rook.index()] & bitboard::from_square(Square::A1),
            bitboard::from_square(Square::A1)
        );
        assert_eq!(
            nbd.by_col[Col::White.index()] & bitboard::from_square(Square::A1),
            bitboard::from_square(Square::A1)
        );
        assert_eq!(
            nbd.occupied & bitboard::from_square(Square::A1),
            bitboard::from_square(Square::A1)
        );
    }
    #[test]
    fn by_col() {
        let nbd = Board::new();
        assert_eq!(nbd.by_col(Col::White), 0xffff);
        assert_eq!(nbd.by_col(Col::Black), 0xffff_0000_0000_0000);
    }
    #[test]
    fn by_piecetype() {
        let nbd = Board::new();
        assert_eq!(nbd.by_piecetype(PieceType::Pawn), 0x00ff_0000_0000_ff00);
        assert_eq!(nbd.by_piecetype(PieceType::Knight), 0x4200_0000_0000_0042);
        assert_eq!(nbd.by_piecetype(PieceType::Bishop), 0x2400_0000_0000_0024);
        assert_eq!(nbd.by_piecetype(PieceType::Rook), 0x8100_0000_0000_0081);
        assert_eq!(nbd.by_piecetype(PieceType::Queen), 0x0800_0000_0000_0008);
        assert_eq!(nbd.by_piecetype(PieceType::King), 0x1000_0000_0000_0010);
    }
    #[test]
    fn by_piece() {
        let nbd = Board::new();
        assert_eq!(nbd.by_piece(Piece::WP), 0x00ff_0000_0000_ff00 & 0xffff);
        assert_eq!(nbd.by_piece(Piece::WN), 0x4200_0000_0000_0042 & 0xffff);
        assert_eq!(nbd.by_piece(Piece::WB), 0x2400_0000_0000_0024 & 0xffff);
        assert_eq!(nbd.by_piece(Piece::WR), 0x8100_0000_0000_0081 & 0xffff);
        assert_eq!(nbd.by_piece(Piece::WQ), 0x0800_0000_0000_0008 & 0xffff);
        assert_eq!(nbd.by_piece(Piece::WK), 0x1000_0000_0000_0010 & 0xffff);
    }
    #[test]
    fn attacks_from() {
        let nbd = Board::new();
        assert_eq!(
            nbd.attacks_from(Square::A1),
            attacks::attacks(Square::A1, Piece::WR, nbd.occupied)
        );
        assert_eq!(
            nbd.attacks_from(Square::E8),
            attacks::attacks(Square::E8, Piece::BK, nbd.occupied)
        );
        assert_eq!(
            nbd.attacks_from(Square::D2),
            attacks::attacks(Square::D2, Piece::WP, nbd.occupied)
        );
        assert_eq!(
            nbd.attacks_from(Square::H8),
            attacks::attacks(Square::H8, Piece::BR, nbd.occupied)
        );
        assert_eq!(nbd.attacks_from(Square::A5), BB_EMPTY);
    }
    #[test]
    fn atacks_to() {
        let nbd = Board::new();
        assert_eq!(nbd.atacks_to(Square::A1, Col::White, nbd.occupied), 0);
        assert_eq!(nbd.atacks_to(Square::E2, Col::White, nbd.occupied), 120);
        assert_eq!(
            nbd.atacks_to(Square::B8, Col::Black, nbd.occupied),
            72057594037927936
        );
        assert_eq!(
            nbd.atacks_to(Square::F6, Col::Black, nbd.occupied),
            4634204016564240384
        );
    }
}
