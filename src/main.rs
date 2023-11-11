
pub mod screen_info;
pub mod minesweeper_utils;
use minesweeper_utils::*;

fn main() {
    let mut grid = Grid::new(5 as u8, 30 as u8);
    grid.stdout();
    grid.mineatize(3 as u8, 8 as u8);
    grid.stdout();
}
