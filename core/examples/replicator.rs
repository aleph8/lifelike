//! Emit HighLife replicator generations as JSON for the README animation.
use lifelike::base::models::Universe;
use lifelike::matrix::{BasicMatrixUniverse, LifeRules};

fn main() {
    let mut universe = BasicMatrixUniverse {
        width: 64,
        height: 48,
        cells: Vec::new(),
        cells_buff: Vec::new(),
        life_rule: LifeRules::HighLife,
        birth_array: [false; 9],
        surv_array: [false; 9],
    };
    universe.new(&[]);
    let rows = ["..OOO", ".O..O", "O...O", "O..O.", "OOO.."];
    for (row, pattern) in rows.iter().enumerate() {
        for (col, cell) in pattern.chars().enumerate() {
            if cell == 'O' {
                universe.set_cell(22 + row as u32, 30 + col as u32, true);
            }
        }
    }

    print!("{{\"width\":64,\"height\":48,\"frames\":[");
    for generation in 0..=84 {
        if generation > 0 {
            print!(",");
        }
        print!("{:?}", universe.cells);
        universe.tick();
    }
    println!("]}}");
}
