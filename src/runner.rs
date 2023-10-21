use std::collections::HashMap;

use crate::types::{Move, TuringDef};

struct Tape {
    tape: HashMap<isize, char>,
    head: isize,
}

impl Tape {
    fn new() -> Self {
        Self {
            tape: HashMap::new(),
            head: 0,
        }
    }

    fn apply_move(&mut self, direction: Move) {
        match direction {
            Move::Left => self.head -= 1,
            Move::Right => self.head += 1,
            Move::None => (),
        }
    }

    fn get(&self) -> &char {
        self.tape.get(&self.head).unwrap_or(&'B')
    }

    fn set(&mut self, value: char) {
        if value == 'B' {
            self.tape.remove(&self.head);
            return;
        }
        self.tape.insert(self.head, value);
    }
}

pub fn run(def: TuringDef) -> anyhow::Result<()> {
    todo!()
}