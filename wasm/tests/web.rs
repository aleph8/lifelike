use lifelike_wasm::{LifeRule, Universe};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn editing_and_reading_packed_cells() {
    let mut u = Universe::new(9, 2, LifeRule::Conway).unwrap();
    assert_eq!((u.width(), u.height()), (9, 2));
    u.set_cell(0, 0, true);
    u.set_cell(0, 8, true);
    u.set_cell(1, 8, true);
    assert!(u.get_cell(0, 8));
    assert_eq!(u.cells(), [128, 128, 64]);
    let copy = u.cells();
    u.set_cell(0, 8, false);
    assert!(!u.get_cell(0, 8));
    assert_eq!(copy, [128, 128, 64]);
    u.set_cell(2, 0, true);
    u.set_cell(0, 9, true);
    assert!(!u.get_cell(2, 0));
    assert!(!u.get_cell(0, 9));
    assert_eq!(u.cells(), [128, 0, 64]);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn blinker_ticks_and_clear_removes_previous_generations() {
    let mut u = Universe::new(5, 5, LifeRule::Conway).unwrap();
    for col in 1..=3 {
        u.set_cell(2, col, true);
    }
    u.tick();
    for row in 0..5 {
        for col in 0..5 {
            assert_eq!(u.get_cell(row, col), col == 2 && (1..=3).contains(&row));
        }
    }
    u.clear();
    u.tick();
    assert!(u.cells().iter().all(|&byte| byte == 0));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn rule_selection_reaches_the_engine() {
    for (rule, expected) in [
        (LifeRule::Conway, false),
        (LifeRule::HighLife, true),
        (LifeRule::Seeds, false),
    ] {
        let mut u = Universe::new(5, 5, rule).unwrap();
        for (row, col) in [(1, 1), (1, 2), (1, 3), (2, 1), (2, 3), (3, 2)] {
            u.set_cell(row, col, true);
        }
        u.tick();
        assert_eq!(u.get_cell(2, 2), expected);
    }
    let mut u = Universe::new(5, 5, LifeRule::Seeds).unwrap();
    u.set_cell(2, 1, true);
    u.set_cell(2, 3, true);
    u.tick();
    assert!(u.get_cell(2, 2));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn empty_dimensions_are_supported() {
    let mut u = Universe::new(0, 5, LifeRule::Conway).unwrap();
    u.set_cell(0, 0, true);
    u.tick();
    u.clear();
    assert!(!u.get_cell(0, 0));
    assert!(u.cells().is_empty());
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
fn overflowing_dimensions_return_an_error() {
    assert!(Universe::new(u32::MAX, 2, LifeRule::Conway).is_err());
}
