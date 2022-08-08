use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub enum Choice {
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
