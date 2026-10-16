use super::models::{LivingCell, Universe};
use crate::matrix::{BasicMatrixUniverse, LifeRules};

fn universe(
    width: u32,
    height: u32,
    rule: LifeRules,
    points: &[(u32, u32)],
) -> BasicMatrixUniverse {
    let mut universe = BasicMatrixUniverse {
        width,
        height,
        cells: Vec::new(),
        cells_buff: Vec::new(),
        life_rule: rule,
        birth_array: [false; 9],
        surv_array: [false; 9],
    };
    seed(&mut universe, points);
    universe
}

fn seed(universe: &mut BasicMatrixUniverse, points: &[(u32, u32)]) {
    let cells: Vec<_> = points
        .iter()
        .map(|&(row, col)| LivingCell { row, col })
        .collect();
    universe.new(&cells);
}

fn living_cells(universe: &BasicMatrixUniverse) -> Vec<(u32, u32)> {
    (0..universe.height())
        .flat_map(|row| (0..universe.width()).map(move |col| (row, col)))
        .filter(|&(row, col)| universe.get_cell_status(row, col, universe.width()))
        .collect()
}

#[test]
fn initialization_preserves_dimensions_and_packs_bits() {
    let points = [(0, 0), (0, 7), (0, 8), (1, 0), (2, 9)];
    let u = universe(10, 3, LifeRules::Conway, &points);
    assert_eq!((u.width(), u.height()), (10, 3));
    assert_eq!(u.cells.len(), 4);
    assert_eq!(u.cells_buff, vec![0; 4]);
    assert_eq!(u.cells, vec![0b10000001, 0b10100000, 0, 0b00000100]);
    assert_eq!(living_cells(&u), points);
}

#[test]
fn initialization_ignores_out_of_bounds_and_duplicate_cells() {
    let u = universe(
        3,
        2,
        LifeRules::Conway,
        &[(1, 2), (1, 2), (2, 0), (0, 3), (u32::MAX, u32::MAX)],
    );
    assert_eq!(living_cells(&u), [(1, 2)]);
}

#[test]
fn reinitialization_clears_both_buffers() {
    let mut u = universe(8, 8, LifeRules::Conway, &[(3, 2), (3, 3), (3, 4)]);
    u.tick();
    seed(&mut u, &[]);
    assert!(living_cells(&u).is_empty());
    assert!(u.cells_buff.iter().all(|&byte| byte == 0));
    u.tick();
    assert!(living_cells(&u).is_empty());
}

#[test]
fn buffer_writes_set_and_clear_bits_without_changing_current_generation() {
    let mut u = universe(9, 2, LifeRules::Conway, &[]);
    u.set_cell_status(0, 7, 9, true);
    u.set_cell_status(0, 8, 9, true);
    u.set_cell_status(1, 8, 9, true);
    u.set_cell_status(0, 8, 9, false);
    assert!(living_cells(&u).is_empty());
    std::mem::swap(&mut u.cells, &mut u.cells_buff);
    assert_eq!(living_cells(&u), [(0, 7), (1, 8)]);
}

#[test]
fn empty_and_zero_sized_universes_remain_empty() {
    for (width, height) in [(8, 8), (0, 8), (8, 0), (0, 0)] {
        let mut u = universe(width, height, LifeRules::Conway, &[]);
        for _ in 0..3 {
            u.tick();
        }
        assert!(living_cells(&u).is_empty());
    }
}

#[test]
fn isolated_cells_die_everywhere_including_all_edges_and_corners() {
    for row in 0..5 {
        for col in 0..5 {
            let mut u = universe(5, 5, LifeRules::Conway, &[(row, col)]);
            assert!(!u.living_cell(row, col), "isolated cell at ({row}, {col})");
            u.tick();
            assert!(
                living_cells(&u).is_empty(),
                "isolated cell at ({row}, {col})"
            );
        }
    }
}

#[test]
fn blocks_are_stable_interior_and_at_every_corner() {
    for (row, col) in [(0, 0), (0, 4), (4, 0), (4, 4), (2, 2)] {
        let points = [
            (row, col),
            (row, col + 1),
            (row + 1, col),
            (row + 1, col + 1),
        ];
        let mut u = universe(6, 6, LifeRules::Conway, &points);
        for _ in 0..4 {
            u.tick();
            assert_eq!(living_cells(&u), points);
        }
    }
}

#[test]
fn blinker_has_period_two() {
    let points = [(3, 2), (3, 3), (3, 4)];
    let mut u = universe(8, 8, LifeRules::Conway, &points);
    for _ in 0..4 {
        u.tick();
        assert_eq!(living_cells(&u), [(2, 3), (3, 3), (4, 3)]);
        u.tick();
        assert_eq!(living_cells(&u), points);
    }
}

#[test]
fn glider_moves_one_cell_diagonally_after_four_generations() {
    let mut u = universe(
        8,
        8,
        LifeRules::Conway,
        &[(2, 3), (3, 4), (4, 2), (4, 3), (4, 4)],
    );
    for _ in 0..4 {
        u.tick();
    }
    assert_eq!(living_cells(&u), [(3, 4), (4, 5), (5, 3), (5, 4), (5, 5)]);
}

#[test]
fn opposite_edges_do_not_wrap() {
    let mut u = universe(5, 5, LifeRules::Conway, &[(0, 0), (0, 4), (4, 0)]);
    u.tick();
    assert!(living_cells(&u).is_empty());
}

#[test]
fn single_row_and_single_column_do_not_duplicate_neighbors() {
    for (width, height, points) in [
        (3, 1, vec![(0, 0), (0, 1), (0, 2)]),
        (1, 3, vec![(0, 0), (1, 0), (2, 0)]),
        (1, 1, vec![(0, 0)]),
    ] {
        let mut u = universe(width, height, LifeRules::Conway, &points);
        u.tick();
        let expected = if width == 3 {
            vec![(0, 1)]
        } else if height == 3 {
            vec![(1, 0)]
        } else {
            vec![]
        };
        assert_eq!(living_cells(&u), expected);
        u.tick();
        assert!(living_cells(&u).is_empty());
    }
}

#[test]
fn all_rules_apply_correctly_for_every_neighbor_count_and_cell_state() {
    let neighbors = [
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 1),
        (2, 3),
        (3, 1),
        (3, 2),
        (3, 3),
    ];
    for rule_id in 0..3 {
        for count in 0..=8 {
            for alive in [false, true] {
                let rule = match rule_id {
                    0 => LifeRules::Conway,
                    1 => LifeRules::HighLife,
                    _ => LifeRules::Seeds,
                };
                let mut points = neighbors[..count].to_vec();
                if alive {
                    points.push((2, 2));
                }
                let mut u = universe(5, 5, rule, &points);
                let expected = if alive {
                    rule_id != 2 && matches!(count, 2 | 3)
                } else {
                    match rule_id {
                        0 => count == 3,
                        1 => matches!(count, 3 | 6),
                        _ => count == 2,
                    }
                };
                assert_eq!(
                    u.living_cell(2, 2),
                    expected,
                    "rule={rule_id}, neighbors={count}, alive={alive}"
                );
                u.tick();
                assert_eq!(u.get_cell_status(2, 2, 5), expected);
            }
        }
    }
}
