mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        // Iterate through 8 neighbors using wrap-around to hit cells to the left or above current
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.width;
                let neighbor_col = (column + delta_col) % self.height;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    pub fn set_alive(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = Cell::Alive;
    }
}

// Public methods for Javascript export
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|_| {
                let v = js_sys::Math::random();
                if v > 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let mut universe = Universe {
            width,
            height,
            cells,
        };

        universe.generate_glider();
        universe
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.get_live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead, // Rule 1: Alive cell with less than 2 neighbors dies by underpopulation
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive, // Rule 2: Alive cell with 2 or 3 neighbors lives
                    (Cell::Alive, x) if x > 3 => Cell::Dead, // Rule 3: Alive cell with over 3 neighbors dies by overpopulation
                    (Cell::Dead, 3) => Cell::Alive, // Rule 4: Dead cell with 3 neighbors lives by reproduction
                    (state, _) => state,            // otherwise: stay the same
                };

                next[idx] = next_cell;
            }
        }

        // Replace current cells with buffer
        self.cells = next;
    }

    pub fn generate_glider(&mut self) {
        self.set_alive(1, 2);
        self.set_alive(2, 3);
        self.set_alive(3, 1);
        self.set_alive(3, 2);
        self.set_alive(3, 3);
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
