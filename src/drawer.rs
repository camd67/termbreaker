use crossterm::cursor;
use crossterm::style;
use crossterm::terminal;
use crossterm::{execute, queue};
use std::io::Write;

use crate::phys::Square;

const ASCII_SQUARE: &str = "█";

pub struct Drawer<W>
where
    W: Write,
{
    w: W,
}

impl<W> Drawer<W>
where
    W: Write,
{
    pub fn new(w: W) -> Drawer<W> {
        Drawer { w }
    }

    /// Enters into the terminal's Alternate screen (make sure to go back after!)
    pub fn enter_alt_screen(&mut self) {
        execute!(self.w, terminal::EnterAlternateScreen).unwrap();
    }

    /// Exits the terminal's Alternate screen and resets most properties
    pub fn exit_alt_screen(&mut self) {
        execute!(
            self.w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )
        .unwrap();
    }

    /// Enters into raw mode.
    /// This let's us consume key events without having to wait for the user to hit "Enter" after
    pub fn enter_raw_mode(&self) {
        terminal::enable_raw_mode().unwrap();
    }

    /// Exits raw mode
    pub fn exit_raw_mode(&self) {
        terminal::disable_raw_mode().unwrap();
    }

    /// Fully resets the screen by:
    /// - Clearing it
    /// - Hiding the cursor
    /// - Moving the cursor to the top left
    /// - Resetting the color
    pub fn reset_screen(&mut self) {
        queue!(
            self.w,
            style::ResetColor,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )
        .unwrap();
    }

    /// Flushes everything in the queue
    pub fn flush_queue(&mut self) {
        self.w.flush().unwrap();
    }

    /// Draws a square of the given color
    pub fn draw_square(&mut self, square: Square, color: style::Color) {
        let row = style::style(ASCII_SQUARE.repeat(square.w as usize)).with(color);
        // print our row of squares, repeating for every row in the square
        // making sure to jump to the next line after each print!
        for i in 0..square.h {
            queue!(
                self.w,
                cursor::MoveTo(square.x + 1, square.y + i + 1),
                style::PrintStyledContent(row.clone())
            )
            .unwrap();
        }
    }
}
