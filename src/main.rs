use std::collections::HashMap;

struct Space {
    dug: bool,
    mine: bool,
    position: (u8, u8) // x, y on the grid where the top left is 0, 0
}

impl Space {

    fn new(x: u8, y: u8) -> Self {
        return Space{dug: false, mine: false, position: (x, y)}
    }

    fn dig(mut self) {
        if !self.dug { //mine has not yet been dug
            if self.mine{
                //you lose
            } else {
                self.dug = true;
            }
        }
    }

    fn get_surrounding_mines(self, grid: Grid) -> u8 {
        let self_col = self.position.0 as i8; // the x
        let self_row = self.position.1 as i8; // the y
        let mut surrounding_mines: u8 = 0;
        
        for row_inc in -1..=1 {
            let checking_row = self_row + row_inc;
            if checking_row < 0 || checking_row >= grid.rows as i8 {
                //its out of bounds, don't check
                continue;
            }

            for col_inc in -1..=1 {
                let checking_col = self_col + col_inc;
                if checking_col < 0 || checking_col >= grid.columns as i8 {
                    //its out of bounds, don't check
                    continue;
                }

                let key = format!("r{}c{}", checking_row, checking_col);
                let space = grid.spaces.get(&key);
                match space {
                    Some(t) => {
                        if t.mine {
                            //if t is a mine
                            surrounding_mines += 1;
                        }
                    },
                    _ => continue
                }
            }
        }

        return surrounding_mines;
    }

}


struct Grid {
    rows: u8,
    columns: u8,
    spaces: HashMap<String, Space>
}

impl Grid {
    fn new(rows: u8, columns: u8) -> Self {
        let mut spaces: HashMap<String, Space> = HashMap::new();
        for row in 0..rows {
            for col in 0..columns {
                let key = format!("r{}c{}", row, col);
                spaces.insert(key, Space::new(col, row));
            }
        }

        return Grid {rows, columns, spaces}
    }

    fn stdout(&self) {
        let mut str = String::new();
        for row in 0..self.rows {
            for col in 0..self.columns {
                let key = format!("r{}c{}", row, col);

                let space = self.spaces.get(&key);

                match space {
                    Some(t) => {
                        if t.mine {
                            str.push('X');
                        } else {
                            let surrounding_mines = t.get_surrounding_mines(self);
                        }
                    },
                    _ => continue
                }
            }

            str.push('\n');
        }
    }
}


fn main() {
    let grid = Grid::new(5 as u8, 5 as u8);
}
