use std::io::{Stdout, Write};
use crossterm::{QueueableCommand, style::{SetBackgroundColor, Color}, terminal::{Clear, ClearType}, cursor::MoveTo};

use crate::frame::Frame;

pub fn render(output: &mut Stdout, last_frame: &Frame, cur_frame: &Frame, force: bool) {
    if force {
        output.queue(SetBackgroundColor(Color::Blue)).unwrap();
        output.queue(Clear(ClearType::All)).unwrap();
        output.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    
    for (x, col) in cur_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                output.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    
    output.flush().unwrap();
}