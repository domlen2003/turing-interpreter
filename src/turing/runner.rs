use std::cmp;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

use super::def::{Move, TuringDef};

struct Tape {
    tape: BTreeMap<isize, char>,
    head: isize,
}

impl Tape {
    fn new() -> Self {
        Self {
            tape: BTreeMap::new(),
            head: 0,
        }
    }

    fn apply_move(&mut self, direction: &Move) {
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

    fn insert(&mut self, index: isize, value: char) {
        if value == 'B' {
            self.tape.remove(&index);
            return;
        }
        self.tape.insert(index, value);
    }

    /**
     * Returns a vector of all values in the tape, starting at the first non-'B' value
     * and ending at the last non-'B' value, always including the head.
     */
    fn as_vec(&self) -> (Vec<&char>, isize) {
        // find the min and max index of the tape
        let max_tape = self.tape.keys().max().unwrap();
        let min_tape = self.tape.keys().min().unwrap();
        // find the min and max index of the tape and the head
        let min = cmp::min(self.head, *min_tape);
        let max = cmp::max(self.head, *max_tape);
        // create a vector of all values in the tape from min to max
        let vec = (min..=max).map(|i| self.tape.get(&i).unwrap_or(&'B')).collect::<Vec<_>>();
        // return the vector and the offset of the first value from 0 position (the min index)
        (vec, min)
    }
}

pub struct TuringRunner {
    def: TuringDef,
    tape: Tape,
    state: u8,
}

impl TuringRunner {
    pub fn new(def: TuringDef) -> Self {
        Self {
            tape: Tape::new(),
            state: def.start_state.clone(),
            def,
        }
    }

    pub fn load_tape(&mut self, value: &str) {
        let tape = &mut self.tape;
        for (i, c) in value.chars().enumerate() {
            if !self.def.input_alphabet.contains(&c) {
                panic!("Invalid character {} in input tape", c);
            }
            tape.insert(i as isize, c);
        }
    }

    pub fn step(&mut self) {
        let current = self.tape.get();
        let transition = self.def.transition_function.iter()
            .find(|&x| x.state == self.state && x.input == *current)
            .unwrap();
        self.tape.set(transition.write);
        self.tape.apply_move(&transition.move_dir);
        self.state = transition.next_state;
    }

    pub fn is_halted(&self) -> bool {
        self.state == self.def.end_state
    }
}

impl Display for TuringRunner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (vec, offset) = self.tape.as_vec();
        for (index, &c) in vec.iter().enumerate() {
            if index as isize + offset == self.tape.head {
                f.write_str(&format!("[q{}]({})", self.state, c))?;
            } else {
                f.write_str(&format!("{}", c))?;
            }
        }
        Ok(())
    }
}

impl From<TuringDef> for TuringRunner {
    fn from(value: TuringDef) -> Self {
        Self::new(value)
    }
}
