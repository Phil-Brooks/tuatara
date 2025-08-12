use std::fmt::{Write, Display};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
#[repr(transparent)]
pub struct BitBoard(pub u64);

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LAST_BIT: u64 = 63;
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let char = if self.0 & mask != 0 { '1' } else { '0' };
                write!(f, "{} ", char)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}    


impl BitBoard {
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    pub fn any(self) -> bool {
        self.0 != 0
    }
    pub fn empty() -> Self {
        Self(0)
    }
}
