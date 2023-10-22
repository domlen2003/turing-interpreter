use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TuringDef {
    // n = #Q = #{q1, q2, ... , qn}
    pub state_count: u8,
    // Σ
    pub input_alphabet: Vec<char>,
    // Γ
    pub tape_alphabet: Vec<char>,
    // q0
    pub start_state: u8,
    // q^
    pub end_state: u8,
    // δ
    pub transition_function: Vec<TransitionFunction>,
}

impl Display for TuringDef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "M = (\n  \
                Q: {},\n  \
                Σ: {},\n  \
                Γ: {},\n  \
                B: B,\n  \
                q0: q{},\n  \
                q^: q{},\n  \
                δ: {}\n\
            )",
            fmt_state_count(self.state_count),
            fmt_char_vec(&self.input_alphabet),
            fmt_char_vec(&self.tape_alphabet),
            self.start_state,
            self.end_state,
            fmt_transition(&self.transition_function)
        ))
    }
}

fn fmt_state_count(count: u8) -> String {
    format!("{{{}}}", (1..=count).map(|x| format!("q{}", x)).collect::<Vec<_>>().join(", "))
}

fn fmt_char_vec(vec: &Vec<char>) -> String {
    format!("{{{}}}", vec.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(", "))
}

fn fmt_transition(vec: &Vec<TransitionFunction>) -> String {
    format!("{{\n    {}\n  }}", vec.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(",\n    "))
}

#[derive(Debug)]
pub struct TransitionFunction {
    // q
    pub state: u8,
    // a
    pub input: char,
    // q'
    pub next_state: u8,
    // b
    pub write: char,
    // L, R or N
    pub move_dir: Move,
}

impl Display for TransitionFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("δ(q{}, {}) = (q{}, {}, {})", self.state, self.input, self.next_state, self.write, self.move_dir))
    }
}

#[derive(Debug)]
pub enum Move {
    Left,
    Right,
    None,
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Move::Left => f.write_str("L"),
            Move::Right => f.write_str("R"),
            Move::None => f.write_str("N"),
        }
    }
}
