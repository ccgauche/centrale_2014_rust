use itertools::Itertools;

use crate::{clause::Clause, f::F, lit::Lit};

fn k1() -> F {
    (0..9)
        .cartesian_product(0..9)
        .map(|(x, y)| (1..10).map(|k| Lit::X(x, y, k)).collect::<Clause>())
        .collect()
}

fn l1() -> F {
    (0..9)
        .cartesian_product(1..10)
        .map(|(x, k)| (0..9).map(|y| Lit::X(x, y, k)).collect::<Clause>())
        .collect()
}

fn c1() -> F {
    (0..9)
        .cartesian_product(1..10)
        .map(|(y, k)| (0..9).map(|x| Lit::X(x, y, k)).collect::<Clause>())
        .collect()
}

fn b1() -> F {
    (0..9)
        .cartesian_product(1..10)
        .map(|(bn, k)| {
            let blockoffsetx = (bn / 3) * 3;
            let blockoffsety = (bn % 3) * 3;
            (0..9)
                .map(|x| Lit::X(blockoffsetx + x / 3, blockoffsety + x % 3, k))
                .collect::<Clause>()
        })
        .collect()
}

fn k2() -> F {
    (0..9)
        .cartesian_product(0..9)
        .cartesian_product(1..9)
        .flat_map(|((x, y), k1)| {
            ((k1 + 1)..10)
                .map(|k2| Clause::new(vec![Lit::NX(x, y, k1), Lit::NX(x, y, k2)]))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn l2() -> F {
    (0..9)
        .cartesian_product(1..10)
        .cartesian_product(0..9)
        .flat_map(|((x, k), y1)| {
            ((y1 + 1)..9)
                .map(|y2| Clause::new(vec![Lit::NX(x, y1, k), Lit::NX(x, y2, k)]))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn c2() -> F {
    (0..9)
        .cartesian_product(1..10)
        .cartesian_product(0..8)
        .flat_map(|((y, k), x1)| {
            ((x1 + 1)..9)
                .map(|x2| Clause::new(vec![Lit::NX(x1, y, k), Lit::NX(x2, y, k)]))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn b2() -> F {
    (0..9)
        .cartesian_product(1..10)
        .cartesian_product(0..8)
        .flat_map(|((bn, k), x1)| {
            let blockoffsetx = (bn / 3) * 3;
            let blockoffsety = (bn % 3) * 3;
            ((x1 + 1)..9)
                .map(|x2| {
                    Clause::new(vec![
                        Lit::NX(blockoffsetx + x1 / 3, blockoffsety + x1 % 3, k),
                        Lit::NX(blockoffsetx + x2 / 3, blockoffsety + x2 % 3, k),
                    ])
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn generate_ruleset() -> F {
    b1().merge(&b2())
        .merge(&c1())
        .merge(&c2())
        .merge(&l1())
        .merge(&l2())
        .merge(&k1())
        .merge(&k2())
}
