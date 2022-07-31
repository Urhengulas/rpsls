use std::io::{self, Write};

fn main() {
    let game = Game::new();
    game.start()
}

struct Game {}

impl Game {
    fn new() -> Game {
        Game {}
    }

    fn start(&self) {
        print!("[0] PVP\n[1] PCV\n[2] Exit\n>>> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        match buf.as_str() {
            "0" => {}
            "1" => {}
            "2" => {}
            _ => {}
        }
    }
}
