use std::{
    cmp::Ordering,
    io::{self, Write},
};

fn main() {
    let game = Game::new();
    game.start()
}

struct Game {}

impl Game {
    fn new() -> Self {
        Self {}
    }

    fn start(&self) {
        let choice = input("[0] PVP\n[1] PVC\n[2] Exit");

        match choice.as_str() {
            "0" => {
                let game = PVP::new([input("Name 1").as_str(), input("Name 2").as_str()]);
                game.start();
            }
            "1" => {}
            "2" => {}
            _ => panic!("{}", choice),
        }
    }
}

struct PVP {
    players: [Player; 2],
}

impl PVP {
    fn new(names: [&str; 2]) -> Self {
        Self {
            players: [Player::new(names[0]), Player::new(names[1])],
        }
    }

    fn start(&self) {
        let idx = loop {
            let [c0, c1] = [self.players[0].choose(), self.players[1].choose()];
            match c0.partial_cmp(&c1) {
                Some(Ordering::Equal) => println!("Draw!\n"),
                Some(Ordering::Greater) => break 0,
                Some(Ordering::Less) => break 1,
                None => unreachable!(),
            }
        };
        println!("{} won!", self.players[idx].name);
    }
}

struct Player {
    name: String,
}

impl Player {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn choose(&self) -> Choice {
        println!("{}", self.name);
        let choice = input("[0] Rock\n[1] Paper\n[2] Sciccors\n[3] Lizard\n[4] Spock");

        match choice.as_str() {
            "0" => Choice::Rock,
            "1" => Choice::Paper,
            "2" => Choice::Sciccors,
            "3" => Choice::Lizard,
            "4" => Choice::Spock,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Sciccors,
    Lizard,
    Spock,
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use {Choice::*, Ordering::*};

        Some(match (self, other) {
            (Sciccors, Paper) => Greater,
            (Paper, Rock) => Greater,
            (Rock, Lizard) => Greater,
            (Lizard, Spock) => Greater,
            (Spock, Sciccors) => Greater,
            (Sciccors, Lizard) => Greater,
            (Lizard, Paper) => Greater,
            (Paper, Spock) => Greater,
            (Spock, Rock) => Greater,
            (Rock, Sciccors) => Greater,

            (Paper, Sciccors) => Less,
            (Rock, Paper) => Less,
            (Lizard, Rock) => Less,
            (Spock, Lizard) => Less,
            (Sciccors, Spock) => Less,
            (Lizard, Sciccors) => Less,
            (Paper, Lizard) => Less,
            (Spock, Paper) => Less,
            (Rock, Spock) => Less,
            (Sciccors, Rock) => Less,

            (Rock, Rock) => Equal,
            (Paper, Paper) => Equal,
            (Sciccors, Sciccors) => Equal,
            (Lizard, Lizard) => Equal,
            (Spock, Spock) => Equal,
        })
    }
}

fn input(s: &str) -> String {
    println!("{}", s);
    print!(">>> ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    println!("");

    buf.trim().to_string()
}
