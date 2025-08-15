use crate::consts::*;

const fn sliding_attacks(square: i32, occupied: u64, deltas: &[i32]) -> u64 {
    let mut attack = 0;

    let mut i = 0;
    let len = deltas.len();
    while i < len {
        let mut previous = square;
        loop {
            let sq = previous + deltas[i];
            let file_diff = (sq & 0x7) - (previous & 0x7);
            if file_diff > 2 || file_diff < -2 || sq < 0 || sq > 63 {
                break;
            }
            let bb = 1 << sq;
            attack |= bb;
            if occupied & bb != 0 {
                break;
            }
            previous = sq;
        }
        i += 1;
    }

    attack
}
const fn bootstrap_stepping_attacks(deltas: &[i32]) -> [u64; 64] {
    let mut table = [0; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = sliding_attacks(sq as i32, !0, deltas);
        sq += 1;
    }
    table
}
static KNIGHT_ATTACKS: [u64; 64] = bootstrap_stepping_attacks(&KNIGHT_DELTAS);
static KING_ATTACKS: [u64; 64] = bootstrap_stepping_attacks(&KING_DELTAS);
static PAWN_ATTACKS: [[u64; 64]; 2] = [
    bootstrap_stepping_attacks(&WHITE_PAWN_DELTAS),
    bootstrap_stepping_attacks(&BLACK_PAWN_DELTAS),
];
pub const fn pawn_attacks(col: Col, sq: Square) -> BitBoard {
    PAWN_ATTACKS[col.index()][sq.index()]
}
pub const fn knight_attacks(sq: Square) -> BitBoard {
    KNIGHT_ATTACKS[sq.index()]
}
pub const fn king_attacks(sq: Square) -> BitBoard {
    KING_ATTACKS[sq.index()]
}
const fn bootstrap_magics() -> [u64; 88772] {
    let mut table = [0; 88772];
    let mut square = 0;
    while square < 64 {
        let magic = &BISHOP_MAGICS[square as usize];
        let range = magic.mask;
        let mut subset = 0;
        loop {
            let attack = sliding_attacks(square, subset, &BISHOP_DELTAS);
            let idx = (magic.factor.wrapping_mul(subset) >> (64 - 9)) as usize + magic.offset;
            assert!(table[idx] == 0 || table[idx] == attack);
            table[idx] = attack;
            subset = subset.wrapping_sub(range) & range;
            if subset == 0 {
                break;
            }
        }

        let magic = &ROOK_MAGICS[square as usize];
        let range = magic.mask;
        let mut subset = 0;
        loop {
            let attack = sliding_attacks(square, subset, &ROOK_DELTAS);
            let idx = (magic.factor.wrapping_mul(subset) >> (64 - 12)) as usize + magic.offset;
            assert!(table[idx] == 0 || table[idx] == attack);
            table[idx] = attack;
            subset = subset.wrapping_sub(range) & range;
            if subset == 0 {
                break;
            }
        }

        square += 1;
    }
    table
}
static ATTACKS: [u64; 88772] = bootstrap_magics();
pub const fn rook_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    let magic = &ROOK_MAGICS[sq.index()];
    let idx =
        ((occupied & magic.mask).wrapping_mul(magic.factor) >> (64 - 12)) as usize + magic.offset;
    ATTACKS[idx]
}
pub const fn bishop_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    let magic = &BISHOP_MAGICS[sq.index()];
    let idx =
        ((occupied & magic.mask).wrapping_mul(magic.factor) >> (64 - 9)) as usize + magic.offset;
    ATTACKS[idx]
}
pub const fn queen_attacks(sq: Square, occupied: BitBoard) -> BitBoard {
    rook_attacks(sq, occupied) ^ bishop_attacks(sq, occupied)
}
pub const fn attacks(sq: Square, piece: Piece, occupied: BitBoard) -> BitBoard {
    match piece.piecetype() {
        PieceType::Pawn => pawn_attacks(piece.col(), sq),
        PieceType::Knight => knight_attacks(sq),
        PieceType::King => king_attacks(sq),
        PieceType::Rook => rook_attacks(sq, occupied),
        PieceType::Bishop => bishop_attacks(sq, occupied),
        PieceType::Queen => queen_attacks(sq, occupied),
    }
}

#[cfg(test)]
mod tests {
    use crate::attacks;
    use crate::consts::*;

    #[test]
    fn pawn_attacks() {
        assert_eq!(attacks::pawn_attacks(Col::White, Square::A2), 131072);
        assert_eq!(attacks::pawn_attacks(Col::White, Square::B2), 327680);
        assert_eq!(attacks::pawn_attacks(Col::Black, Square::A7), 2199023255552);
        assert_eq!(attacks::pawn_attacks(Col::Black, Square::B7), 5497558138880);
    }
    #[test]
    fn knight_attacks() {
        assert_eq!(attacks::knight_attacks(Square::A1), 132096);
        assert_eq!(attacks::knight_attacks(Square::D1), 1319424);
        assert_eq!(attacks::knight_attacks(Square::E1), 2638848);
        assert_eq!(attacks::knight_attacks(Square::H1), 4202496);
    }
    #[test]
    fn king_attacks() {
        assert_eq!(attacks::king_attacks(Square::A1), 770);
        assert_eq!(attacks::king_attacks(Square::D1), 7188);
        assert_eq!(attacks::king_attacks(Square::E1), 14376);
        assert_eq!(attacks::king_attacks(Square::H1), 49216);
    }
    #[test]
    fn rook_attacks() {
        let occupied = 0;
        assert_eq!(
            attacks::rook_attacks(Square::A1, occupied),
            72340172838076926
        );
        assert_eq!(attacks::rook_attacks(Square::A1, 4), 72340172838076678);
        assert_eq!(attacks::rook_attacks(Square::A1, 65540), 65798);
        assert_eq!(attacks::rook_attacks(Square::A1, 258), 258);
    }
    #[test]
    fn bishop_attacks() {
        let occupied = 0;
        assert_eq!(attacks::bishop_attacks(Square::D1, occupied), 550848566272);
        assert_eq!(attacks::bishop_attacks(Square::D1, 131072), 550831789056);
        assert_eq!(attacks::bishop_attacks(Square::D1, 2228224), 2233344);
        assert_eq!(attacks::bishop_attacks(Square::D1, 5120), 5120);
    }
    #[test]
    fn queen_attacks() {
        let occupied = 0;
        assert_eq!(
            attacks::queen_attacks(Square::A1, occupied),
            9313761861428380670
        );
        assert_eq!(
            attacks::queen_attacks(Square::A1, 65536),
            9241421688590369790
        );
        assert_eq!(attacks::queen_attacks(Square::A1, 327680), 328702);
        assert_eq!(attacks::queen_attacks(Square::A1, 327684), 328454);
        assert_eq!(attacks::queen_attacks(Square::A1, 770), 770);
    }
    #[test]
    fn attacks() {
        let occupied = 0;
        assert_eq!(
            attacks::attacks(
                Square::A1,
                Piece::new(Col::White, PieceType::Pawn),
                occupied
            ),
            512
        );
        assert_eq!(
            attacks::attacks(
                Square::A1,
                Piece::new(Col::White, PieceType::Knight),
                occupied
            ),
            132096
        );
        assert_eq!(
            attacks::attacks(
                Square::A1,
                Piece::new(Col::White, PieceType::King),
                occupied
            ),
            770
        );
        assert_eq!(
            attacks::attacks(
                Square::A1,
                Piece::new(Col::White, PieceType::Rook),
                occupied
            ),
            72340172838076926
        );
        assert_eq!(
            attacks::attacks(
                Square::A1,
                Piece::new(Col::White, PieceType::Bishop),
                occupied
            ),
            9241421688590303744
        );
        assert_eq!(
            attacks::attacks(
                Square::A1,
                Piece::new(Col::White, PieceType::Queen),
                occupied
            ),
            9313761861428380670
        );
    }
}
