use std::fmt::Display;

use itertools::Itertools;

use crate::lit::Lit;

#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct Clause(pub Vec<Lit>);

impl Clause {
    pub fn new(lits: Vec<Lit>) -> Self {
        Self(lits)
    }

    pub fn contains(&self, lit: &Lit) -> bool {
        self.0.contains(lit)
    }

    pub fn clean_of(&mut self, lit: &Lit) {
        self.0.retain(|l| l != lit);
    }
}

impl Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}}}", self.0.iter().map(Lit::to_string).join(""))
    }
}

impl FromIterator<Lit> for Clause {
    fn from_iter<T: IntoIterator<Item = Lit>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
