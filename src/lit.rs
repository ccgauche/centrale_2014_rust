use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lit {
    NX(u8, u8, u8),
    X(u8, u8, u8),
}

impl Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lit::NX(x, y, z) => write!(f, "!({},{},{})", x, y, z),
            Lit::X(x, y, z) => write!(f, "({},{},{})", x, y, z),
        }
    }
}

impl From<(u8, u8, u8)> for Lit {
    fn from(e: (u8, u8, u8)) -> Self {
        Lit::X(e.0, e.1, e.2)
    }
}

impl From<Lit> for (u8, u8, u8) {
    fn from(e: Lit) -> Self {
        match e {
            Lit::NX(x, y, z) => (x, y, z),
            Lit::X(x, y, z) => (x, y, z),
        }
    }
}

impl Lit {
    pub fn neg(&self) -> Self {
        match *self {
            Self::NX(a, b, c) => Self::X(a, b, c),
            Self::X(a, b, c) => Self::NX(a, b, c),
        }
    }
}
