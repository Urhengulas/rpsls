mod choice;
mod computer;
mod human;

use std::{
    cmp::Ordering,
    io::{self, Write},
    process,
};

use crate::{choice::Choice, computer::Computer, human::Human};

fn main() {
    let game = Menu::new();
    game.start()
}

struct Menu;

impl Menu {
    fn new() -> Self {
        Self
    }

    fn start(&self) {
        let choice = input("[0] PVP\n[1] PVC\n[2] CVC\n[3] Exit");

        let mut game = match choice.as_str() {
            "0" => Game::pvp(input("Name 1").as_str(), input("Name 2").as_str()),
            "1" => Game::pvc(input("Name 1").as_str()),
            "2" => Game::cvc(),
            "3" => {
                println!("Good bye!");
                return;
            }
            _invalid => invalid(_invalid),
        };

        loop {
            game.start();

            match input("Play another round? y/n").as_str() {
                "y" => continue,
                "n" => break,
                _invalid => invalid(_invalid),
            }
        }
    }
}

struct Game {
    players: [(Box<dyn Player>, u32); 2],
}

impl Game {
    fn pvp(name_1: &str, name_2: &str) -> Self {
        Self {
            players: [
                (Box::new(Human::new(name_1)), 0),
                (Box::new(Human::new(name_2)), 0),
            ],
        }
    }

    fn pvc(name: &str) -> Self {
        Self {
            players: [
                (Box::new(Human::new(name)), 0),
                (Box::new(Computer::new()), 0),
            ],
        }
    }

    fn cvc() -> Self {
        Self {
            players: [
                (Box::new(Computer::new()), 0),
                (Box::new(Computer::new()), 0),
            ],
        }
    }

    fn start(&mut self) {
        let idx = loop {
            let [c0, c1] = [self.players[0].0.choose(), self.players[1].0.choose()];
            println!(
                "{}: {c0:?}\n{}: {c1:?}\n",
                self.players[0].0.name(),
                self.players[1].0.name()
            );
            match c0.partial_cmp(&c1) {
                Some(Ordering::Equal) => println!("Draw!\n"),
                Some(Ordering::Greater) => break 0,
                Some(Ordering::Less) => break 1,
                None => unreachable!(),
            }
        };
        self.players[idx].1 += 1;
        println!("{} won!\n", self.players[idx].0.name());
        println!(
            "{}:{}\n{:pad0$}:{}",
            self.players[0].0.name(),
            self.players[1].0.name(),
            self.players[0].1,
            self.players[1].1,
            pad0 = self.players[0].0.name().len(),
        )
    }
}

pub trait Player {
    fn choose(&mut self) -> Choice;
    fn name(&self) -> String;
}

fn input(s: &str) -> String {
    print!("{s}\n>>> ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    println!();

    buf.trim().to_string()
}

fn invalid(s: &str) -> ! {
    println!("Invalid choice \"{}\"", s);
    process::exit(0);
}
