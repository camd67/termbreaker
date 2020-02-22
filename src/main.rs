use crate::drawer::Drawer;
use crossterm::event::{poll, read, Event, KeyCode};
use std::io::stdout;
use std::time::Duration;

mod breaker;
mod drawer;
mod phys;

const QUIT_KEY: KeyCode = KeyCode::Char('q');

fn main() {
    let mut game = breaker::Game::new(crossterm::terminal::size().unwrap());
    let mut drawer = Drawer::new(stdout());

    drawer.enter_alt_screen();
    drawer.enter_raw_mode();

    let mut game_should_continue = true;
    while game_should_continue {
        let mut pressed_key: Option<KeyCode> = Option::None;
        if poll(Duration::from_millis(10)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        // Some special keys are handled here
                        QUIT_KEY => break,
                        // All others pass to the game itself
                        _ => pressed_key = Option::Some(event.code),
                    }
                }
                // We don't really care about mouse or resize events... for now
                _ => {}
            }
        }

        drawer.reset_screen();

        game_should_continue = game.update(pressed_key);

        game.draw(&mut drawer);

        drawer.flush_queue();
    }

    drawer.exit_alt_screen();
    drawer.exit_raw_mode();
}
