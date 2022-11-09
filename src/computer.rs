use std::sync::atomic::{AtomicU8, Ordering};

use rand::prelude::*;

use crate::{choice::Choice, Player};

static COUNTER: AtomicU8 = AtomicU8::new(0);

#[derive(Debug)]
pub struct Computer {
    id: u8,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            id: COUNTER.fetch_add(1, Ordering::AcqRel),
        }
    }
}

impl Player for Computer {
    fn choose(&mut self) -> crate::choice::Choice {
        match thread_rng().gen_range(0..5) {
            0 => Choice::Lizard,
            1 => Choice::Paper,
            2 => Choice::Rock,
            3 => Choice::Sciccors,
            4 => Choice::Spock,
            _ => unreachable!(),
        }
    }

    fn name(&self) -> String {
        format!("Computer {}", self.id)
    }
}

impl Drop for Computer {
    fn drop(&mut self) {
        COUNTER.fetch_sub(1, Ordering::AcqRel);
    }
}
