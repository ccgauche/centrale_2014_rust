use std::fmt::Display;

use crate::{clause::Clause, f::F, rule::generate_ruleset};

pub struct Grid(Vec<Vec<u8>>);

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (a, row) in self.0.iter().enumerate() {
            for (b, col) in row.iter().enumerate() {
                write!(f, "{} ", col)?;
                if b % 3 == 2 && b != 8 {
                    write!(f, "| ")?;
                }
            }
            writeln!(f)?;
            if a % 3 == 2 && a != 8 {
                writeln!(f, "---------------------")?;
            }
        }
        Ok(())
    }
}

impl Grid {
    pub fn new() -> Self {
        Self((0..9).map(|_| (0..9).map(|_| 0).collect()).collect())
    }

    pub fn set(&mut self, x: u8, y: u8, k: u8) {
        self.0[x as usize][y as usize] = k;
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        self.0[x as usize][y as usize]
    }

    pub fn apply_sudoku_rules(&mut self) {
        self.apply_on_grid(&mut self.f1(generate_ruleset()));
    }

    pub fn apply_on_grid(&mut self, f: &mut F) {
        'a: loop {
            f.unary_propagation(|x, y, k| self.set(x, y, k));
            if let Some(e) = f
                .vars()
                .iter()
                .find_map(|x| f.get_simplification_factor(*x))
            {
                f.add(Clause::new(vec![e]));
                continue 'a;
            }
            break;
        }
    }

    // F2 is also encoded here in the if statement t == 0
    // Update the F passed as an argument
    pub fn f1(&self, mut f: F) -> F {
        for x in 0..9 {
            for y in 0..9 {
                let t = self.get(x, y);
                if t == 0 {
                    let (bx, by) = (x / 3 * 3, y / 3 * 3);
                    for j in 0..9 {
                        if j != y && self.get(x, j) != 0 {
                            f.add_n(x, y, self.get(x, j));
                        }
                        if j != x && self.get(j, y) != 0 {
                            f.add_n(x, y, self.get(j, y));
                        }
                        let (nx, ny) = (bx + j % 3, by + j / 3);
                        if nx != x && ny != y && self.get(nx, ny) != 0 {
                            f.add_n(x, y, self.get(nx, ny));
                        }
                    }
                    continue;
                }
                for k in 1..10 {
                    if t == k {
                        f.add_p(x, y, k);
                    } else {
                        f.add_n(x, y, k);
                    }
                }
            }
        }
        f
    }
}
