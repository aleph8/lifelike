//! A leaf-shaped HighLife seed, emitting frames computed by the Rust engine.
use lifelike::base::models::Universe;
use lifelike::matrix::{BasicMatrixUniverse, LifeRules};

fn main() {
    let mut universe = BasicMatrixUniverse {
        width: 96,
        height: 96,
        cells: Vec::new(),
        cells_buff: Vec::new(),
        life_rule: LifeRules::HighLife,
        birth_array: [false; 9],
        surv_array: [false; 9],
    };
    universe.new(&[]);
    let rows = [
        "......OO", "....OO.O", "...O..OO", "..O.O.O.", "..OO.O..", ".O.OO...", ".OO.....",
        "O.......",
    ];
    for (row, pattern) in rows.iter().enumerate() {
        for (col, cell) in pattern.chars().enumerate() {
            if cell == 'O' {
                universe.set_cell(44 + row as u32, 44 + col as u32, true);
            }
        }
    }
    print!(
        "{{\"width\":96,\"height\":96,\"scale\":6,\"title\":\"LEAF / HIGHLIFE\",\"rule\":\"B36/S23\",\"duration\":100,\"pause\":1500,\"frames\":["
    );
    for generation in 0..=160 {
        if generation > 0 {
            print!(",");
        }
        print!("{:?}", universe.cells);
        universe.tick();
    }
    println!("]}}");
}
