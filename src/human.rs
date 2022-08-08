use crate::{input, invalid, Choice, Player};

pub struct Human {
    pub name: String,
}

impl Human {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Player for Human {
    fn choose(&mut self) -> Choice {
        println!("{}", self.name);
        let choice = input("[0] Rock\n[1] Paper\n[2] Sciccors\n[3] Lizard\n[4] Spock");

        match choice.as_str() {
            "0" => Choice::Rock,
            "1" => Choice::Paper,
            "2" => Choice::Sciccors,
            "3" => Choice::Lizard,
            "4" => Choice::Spock,
            _invalid => invalid(_invalid),
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
