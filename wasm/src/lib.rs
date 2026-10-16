use lifelike::base::models::Universe as _;
use lifelike::matrix::{BasicMatrixUniverse, LifeRules};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum LifeRule {
    Conway,
    HighLife,
    Seeds,
}

#[wasm_bindgen]
pub struct Universe {
    inner: BasicMatrixUniverse,
}

#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, rule: LifeRule) -> Result<Universe, JsError> {
        width
            .checked_mul(height)
            .and_then(|size| size.checked_add(7))
            .ok_or_else(|| JsError::new("Universe dimensions are too large"))?;
        let mut inner = BasicMatrixUniverse {
            width,
            height,
            cells: Vec::new(),
            cells_buff: Vec::new(),
            life_rule: match rule {
                LifeRule::Conway => LifeRules::Conway,
                LifeRule::HighLife => LifeRules::HighLife,
                LifeRule::Seeds => LifeRules::Seeds,
            },
            birth_array: [false; 9],
            surv_array: [false; 9],
        };
        inner.new(&[]);
        Ok(Self { inner })
    }

    pub fn width(&self) -> u32 {
        self.inner.width
    }

    pub fn height(&self) -> u32 {
        self.inner.height
    }

    #[wasm_bindgen(js_name = getCell)]
    pub fn get_cell(&self, row: u32, col: u32) -> bool {
        row < self.height()
            && col < self.width()
            && self.inner.get_cell_status(row, col, self.width())
    }

    #[wasm_bindgen(js_name = setCell)]
    pub fn set_cell(&mut self, row: u32, col: u32, value: bool) {
        self.inner.set_cell(row, col, value);
    }

    pub fn tick(&mut self) {
        self.inner.tick();
    }

    pub fn clear(&mut self) {
        self.inner.cells.fill(0);
        self.inner.cells_buff.fill(0);
    }

    /// Returns a copy, in row order, with the first cell in the highest bit.
    pub fn cells(&self) -> Vec<u8> {
        self.inner.cells.clone()
    }
}
