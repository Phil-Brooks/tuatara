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
static PAWN_ATTACKS: [[u64; 64];2] = [
    bootstrap_stepping_attacks(&WHITE_PAWN_DELTAS),
    bootstrap_stepping_attacks(&BLACK_PAWN_DELTAS),
];
pub const fn pawn_attacks(col: Col, sq: Square) -> BitBoard {
    PAWN_ATTACKS[col.index()][sq.index()]
}
pub const fn knight_attacks(sq: Square) -> BitBoard {
    KNIGHT_ATTACKS[sq.index()]
}

#[cfg(test)]
mod tests {
    use crate::consts::*;
    use crate::attacks;

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
}   
