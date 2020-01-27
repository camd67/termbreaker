use std::io::stdout;

mod screen_handler;

fn main() {
    let mut screen = screen_handler::Handler::new(stdout());

    screen.init();
    screen.run_loop();
    screen.shutdown();
}
