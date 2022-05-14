#![feature(bool_to_option)]

use crate::grid::Grid;

mod clause;
mod f;
mod grid;
mod lit;
mod rule;

fn main() {
    let mut grid = Grid::new();
    // L1
    grid.set(0, 1, 9);
    grid.set(0, 3, 2);
    grid.set(0, 6, 6);
    grid.set(0, 8, 5);
    // L2
    grid.set(1, 0, 3);
    grid.set(1, 1, 2);
    grid.set(1, 5, 7);
    // L3
    grid.set(2, 1, 7);
    grid.set(2, 3, 9);
    grid.set(2, 5, 5);
    grid.set(2, 8, 8);
    // L4
    grid.set(3, 1, 1);
    // L5
    grid.set(4, 2, 7);
    grid.set(4, 7, 9);
    grid.set(4, 8, 4);
    // L6
    grid.set(5, 0, 6);
    // L7
    grid.set(6, 2, 8);
    grid.set(6, 8, 7);
    // L8
    grid.set(7, 1, 3);
    grid.set(7, 3, 4);
    grid.set(7, 4, 9);
    grid.set(7, 5, 1);
    grid.set(7, 6, 5);
    // L9
    grid.set(8, 5, 3);
    println!("{grid}");
    grid.apply_sudoku_rules();
    println!("{grid}");
}
