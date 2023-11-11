use std::{collections::HashMap, process};

pub struct Space {
    dug: bool,
    mine: bool,
    position: (u8, u8) // x, y on the grid where the top left is 0, 0
}

impl Space {

    pub fn new(x: u8, y: u8) -> Self {
        return Space{dug: false, mine: false, position: (x, y)}
    }
    
    /**
     * Attempts to dig a mine, ending the game if you dig the wrong one
     */
    pub fn dig(&mut self) {
        if !self.dug { //mine has not yet been dug
            if self.mine{
                //you lose
                process::exit(0);
            } else {
                self.dug = true;
            }
        }
    }

    /**
    O(1) implementation omg
     */
    fn get_surrounding_mines(&self, grid: &Grid) -> u8 {
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


pub struct Grid {
    rows: u8,
    columns: u8,
    spaces: HashMap<String, Space>
}


impl Grid {
    pub fn new(rows: u8, columns: u8) -> Self {
        let mut spaces: HashMap<String, Space> = HashMap::new();
        for row in 0..rows {
            for col in 0..columns {
                let key = format!("r{}c{}", row, col);
                let mut space = Space::new(col, row);

                spaces.insert(key, space);
            }
        }

        return Grid {rows, columns, spaces}
    }

    pub fn mineatize(&mut self, p_row: u8, p_col: u8) {
        for row in 0..self.rows {
            for col in 0..self.columns {
                if (p_row as i8 - row as i8).abs() <= 1 && (p_col as i8 - col as i8).abs() <= 1 {
                    //you're too close to the pivot, dont make this a mine!
                    continue;
                }
                let key = format!("r{}c{}", row, col);
                if rand::random() && self.spaces.get(&key).is_some() {
                    let mut space = Space::new(col, row);
                    space.mine = true;
                    self.spaces.insert(key, space);
                }
            }
        }
    }

    pub fn stdout(&self) {
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
                            str.push_str(&surrounding_mines.to_string());
                        }
                        str.push(' ');
                    },
                    _ => continue
                }
            }

            str.push('\n');
        }
        println!("{str}");
    }
}
