use crate::consts::*;
impl Board {
    pub fn new() -> Self {
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
    pub fn empty() -> Self {
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

        if by_col[Colour::Black.index()] & by_col[Colour::White.index()] != BB_EMPTY {
            return Err("colors overlap");
        }

        if occupied != by_col[Colour::Black.index()] | by_col[Colour::White.index()] {
            return Err("roles and colors are mismatched");
        }

        Ok(Board {
            by_piece,
            by_col,
            occupied,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    #[test]
    fn new() {
        let board = Board::new();
        assert_eq!(
            board.by_piece[PieceType::Pawn.index()],
            0x00ff_0000_0000_ff00
        );
        assert_eq!(board.by_col[Colour::White.index()], 0xffff);
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
        assert_eq!(err, "roles and colors are mismatched");
        by_col = [BB_FULL, BB_RANK_1];
        board = Board::try_from_bitboards(by_piece, by_col);
        let err = board.unwrap_err();
        assert_eq!(err, "colors overlap");
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
}
