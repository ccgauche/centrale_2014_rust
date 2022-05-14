use std::fmt::Display;

use itertools::Itertools;

use crate::{clause::Clause, lit::Lit};

#[derive(Default, Clone, PartialEq)]
pub struct F(Vec<Clause>);

impl Display for F {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(Clause::to_string).join("^"))
    }
}

impl F {
    pub fn contains(&self, clause: &Clause) -> bool {
        self.0.contains(clause)
    }
    pub fn merge(&mut self, other: &Self) -> Self {
        self.0
            .iter()
            .chain(other.0.iter())
            .unique()
            .cloned()
            .collect()
    }
    pub fn get_simplification_factor(&self, var_to_test: (u8, u8, u8)) -> Option<Lit> {
        [Lit::from(var_to_test), Lit::from(var_to_test).neg()]
            .into_iter()
            .find_map(|x| {
                let mut copy = self.clone();
                copy.simplify(&x);
                copy.unary_propagation(|_, _, _| {});
                copy.0.is_empty().then_some(x.neg())
            })
    }

    pub fn vars(&self) -> Vec<(u8, u8, u8)> {
        self.0
            .iter()
            .flat_map(|c| c.0.iter().copied().map(Lit::into))
            .unique()
            .collect()
    }
    pub fn add(&mut self, clause: Clause) {
        if !self.contains(&clause) {
            self.0.push(clause);
        }
    }
    pub fn add_n(&mut self, x: u8, y: u8, k: u8) {
        self.add(Clause::new(vec![Lit::NX(x, y, k)]));
    }
    pub fn add_p(&mut self, x: u8, y: u8, k: u8) {
        self.add(Clause::new(vec![Lit::X(x, y, k)]));
    }
    pub fn alone(&self) -> Option<Lit> {
        self.0.iter().find(|x| x.0.len() == 1).map(|x| x.0[0])
    }
    pub fn simplify(&mut self, lit: &Lit) {
        self.0.iter_mut().for_each(|x| x.clean_of(&lit.neg()));
        self.0.retain(|x| !x.contains(lit) && !x.0.is_empty());
    }
    pub fn unary_propagation(&mut self, mut when_found: impl FnMut(u8, u8, u8)) {
        while let Some(e) = self.alone() {
            self.simplify(&e);
            if let Lit::X(x, y, k) = e {
                when_found(x, y, k);
            }
        }
    }
}

impl FromIterator<Clause> for F {
    fn from_iter<T: IntoIterator<Item = Clause>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
