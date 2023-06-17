use std::time::Duration;

use crate::{NUM_COLS, NUM_ROWS, frame::{Drawable, Frame}, shot::Shot};

const MAX_SHOTS: usize = 10;

pub struct Player {
    x: usize,
    y: usize,
}

impl Player {
    pub fn new () -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
    }
}