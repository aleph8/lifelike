use crate::base::models::{LivingCell, Universe};
use regex::Regex;
use std::mem;

pub enum LifeRules {
    Conway,
    HighLife,
    Seeds,
}

pub struct BasicMatrixUniverse {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<u8>,
    pub cells_buff: Vec<u8>,
    pub life_rule: LifeRules,
    pub birth_array: [bool; 9],
    pub surv_array: [bool; 9],
}

impl BasicMatrixUniverse {
    /// Edits the current generation. Coordinates outside the board are ignored.
    pub fn set_cell(&mut self, row: u32, col: u32, value: bool) {
        if row >= self.height || col >= self.width {
            return;
        }
        let position = self.width * row + col;
        let mask = 1 << (7 - position % 8);
        let byte = &mut self.cells[(position / 8) as usize];
        if value {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
}

fn parse_rules(exp: &str) -> ([bool; 9], [bool; 9]) {
    let rules_re = Regex::new(r"^B([0-8]*)/S([0-8]*)$").unwrap();

    let caps = match rules_re.captures(exp) {
        Some(c) => c,
        None => return parse_rules("B3/S23"),
    };

    let mut birth_map = [false; 9];
    let mut survive_map = [false; 9];

    for ch in caps[1].chars() {
        if let Some(digit) = ch.to_digit(10) {
            birth_map[digit as usize] = true;
        }
    }

    for ch in caps[2].chars() {
        if let Some(digit) = ch.to_digit(10) {
            survive_map[digit as usize] = true;
        }
    }

    (birth_map, survive_map)
}

impl Universe for BasicMatrixUniverse {
    fn height(&self) -> u32 {
        return self.height;
    }
    fn width(&self) -> u32 {
        return self.width;
    }
    fn living_cell(&self, row: u32, col: u32) -> bool {
        let current_cell_is_living: bool = self.get_cell_status(row, col, self.width);
        let mut living_neighbors: usize = 0;

        for row_neighbor in -1..=1 {
            for col_neighbor in -1..=1 {
                if row_neighbor == 0 && col_neighbor == 0 {
                    continue;
                }

                let Some(n_row) = row.checked_add_signed(row_neighbor) else {
                    continue;
                };
                let Some(n_col) = col.checked_add_signed(col_neighbor) else {
                    continue;
                };

                if n_row < self.height && n_col < self.width {
                    if self.get_cell_status(n_row, n_col, self.width) {
                        living_neighbors += 1;
                    }
                }
            }
        }

        if current_cell_is_living {
            self.surv_array[living_neighbors]
        } else {
            self.birth_array[living_neighbors]
        }
    }
    fn get_cell_status(&self, row: u32, col: u32, universe_width: u32) -> bool {
        let abs_position: u32 = universe_width * row + col;
        let u8_memory_slot: usize = (abs_position / 8) as usize;
        let position_in_slot: u32 = abs_position % 8;
        let cell_status: u8 = (self.cells[u8_memory_slot] >> (7 - position_in_slot)) & 1;
        cell_status == 1
    }
    fn set_cell_status(&mut self, row: u32, col: u32, universe_width: u32, value: bool) {
        let abs_position: u32 = universe_width * row + col;
        let u8_memory_slot: usize = (abs_position / 8) as usize;
        let position_in_slot: u32 = abs_position % 8;

        let mask: u8 = 1 << (7 - position_in_slot);
        if value {
            self.cells_buff[u8_memory_slot] |= mask;
        } else {
            self.cells_buff[u8_memory_slot] &= !mask;
        }
    }
    fn new(&mut self, starter_cells: &[LivingCell]) {
        let universe_dim = ((self.height * self.width + 7) / 8) as usize;
        self.cells = vec![0; universe_dim];
        self.cells_buff = vec![0; universe_dim];
        for cell in starter_cells {
            if cell.row >= self.height {
                continue;
            }
            if cell.col >= self.width {
                continue;
            }
            self.set_cell_status(cell.row, cell.col, self.width, true);
        }
        mem::swap(&mut self.cells, &mut self.cells_buff);

        let rules_exp = match self.life_rule {
            LifeRules::Conway => "B3/S23",
            LifeRules::HighLife => "B36/S23",
            LifeRules::Seeds => "B2/S",
        };

        let (birth, survive) = parse_rules(rules_exp);

        self.birth_array = birth;
        self.surv_array = survive;
    }
    fn tick(&mut self) {
        for universe_column in 0..self.width {
            for universe_row in 0..self.height {
                if self.living_cell(universe_row, universe_column) {
                    self.set_cell_status(universe_row, universe_column, self.width, true);
                } else {
                    self.set_cell_status(universe_row, universe_column, self.width, false);
                }
            }
        }
        mem::swap(&mut self.cells, &mut self.cells_buff);
    }
}
