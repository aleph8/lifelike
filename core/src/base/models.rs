pub struct LivingCell {
    pub row: u32,
    pub col: u32
}

pub trait Universe {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn living_cell(&self, row: u32, col: u32) -> bool;
    fn get_cell_status(&self, row: u32, col: u32, universe_width: u32) -> bool;
    fn set_cell_status(&mut self, row: u32, col: u32, universe_width: u32, value: bool);
    fn new(&mut self, starter_cells: &[LivingCell]);
    fn tick(&mut self);
}