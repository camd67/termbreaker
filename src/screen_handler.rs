use std::io::Write;
use std::time::Duration;

use crossterm::{execute, queue};
use crossterm::cursor;
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::style;
use crossterm::terminal;

pub struct Handler<W> where W: Write {
    w: W,
}

impl<W> Handler<W> where W: Write {

    /// Creates a new Handler with the given writer
    pub fn new(w: W) -> Handler<W> {
        Handler {
            w,
        }
    }

    /// Initializes the stored writer to prepare for writing
    pub fn init(&mut self) {
        execute!(self.w, terminal::EnterAlternateScreen).expect("Couldn't enter alternate screen on init");
        terminal::enable_raw_mode().expect("Couldn't enable terminal raw mode");
    }

    /// Runs the main loop, publishing key events
    pub fn run_loop(&mut self) {
        loop {
            queue!(self.w,
                style::ResetColor,
                terminal::Clear(terminal::ClearType::All),
                cursor::Hide,
                cursor::MoveTo(1,1)
            ).expect("Couldn't run clear queue during main loop");

            self.w.flush().expect("Couldn't flush the stream on run_loop");

            if poll(Duration::from_millis(500)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => {
                        match event.code {
                            KeyCode::Char('q') => break,
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// Cleans up the writer
    pub fn shutdown(&mut self) {
        execute!(self.w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        ).expect("Couldn't cleanup terminal state during shutdown");
        terminal::disable_raw_mode().expect("Could not disable raw mode! This could leave the terminal in a bad state");
    }
}

