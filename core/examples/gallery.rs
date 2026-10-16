//! Emit ConwayLife gallery patterns as engine-computed JSON frames.
use lifelike::base::models::Universe;
use lifelike::matrix::{BasicMatrixUniverse, LifeRules};

fn main() {
    let name = std::env::args()
        .nth(1)
        .expect("Pass a gallery pattern name");
    let (title, rle, frames, top, left, duration, pause) = match name.as_str() {
        "glider" => (
            "GLIDER",
            include_str!("../../docs/patterns/glider.rle"),
            49,
            12,
            18,
            120,
            1000,
        ),
        "pulsar" => (
            "PULSAR",
            include_str!("../../docs/patterns/pulsar.rle"),
            12,
            18,
            26,
            250,
            250,
        ),
        "pentadecathlon" => (
            "PENTADECATHLON",
            include_str!("../../docs/patterns/pentadecathlon.rle"),
            30,
            23,
            27,
            140,
            140,
        ),
        "lwss" => (
            "LIGHTWEIGHT SPACESHIP",
            include_str!("../../docs/patterns/lwss.rle"),
            49,
            22,
            48,
            120,
            1000,
        ),
        "gosper-glider-gun" => (
            "GOSPER GLIDER GUN",
            include_str!("../../docs/patterns/gosper-glider-gun.rle"),
            121,
            3,
            3,
            80,
            1000,
        ),
        _ => panic!("Unknown gallery pattern: {name}"),
    };
    let mut universe = BasicMatrixUniverse {
        width: 64,
        height: 48,
        cells: Vec::new(),
        cells_buff: Vec::new(),
        life_rule: LifeRules::Conway,
        birth_array: [false; 9],
        surv_array: [false; 9],
    };
    universe.new(&[]);
    let mut row = top;
    let mut col = left;
    let mut count = 0;
    // Only the RLE body is needed; every gallery pattern uses Conway's rule.
    for ch in rle
        .lines()
        .filter(|line| !line.starts_with('#') && !line.starts_with('x'))
        .flat_map(str::chars)
    {
        if let Some(digit) = ch.to_digit(10) {
            count = count * 10 + digit;
            continue;
        }
        if ch.is_whitespace() {
            continue;
        }
        let run = if count == 0 { 1 } else { count };
        count = 0;
        match ch {
            'b' => col += run,
            'o' => {
                for _ in 0..run {
                    universe.set_cell(row, col, true);
                    col += 1;
                }
            }
            '$' => {
                row += run;
                col = left;
            }
            '!' => break,
            _ => panic!("Unexpected RLE character: {ch}"),
        }
    }
    print!(
        "{{\"width\":64,\"height\":48,\"title\":\"{title}\",\"rule\":\"B3/S23\",\"duration\":{duration},\"pause\":{pause},\"frames\":["
    );
    for generation in 0..frames {
        if generation > 0 {
            print!(",");
        }
        print!("{:?}", universe.cells);
        universe.tick();
    }
    println!("]}}");
}
