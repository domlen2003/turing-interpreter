
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

#[derive(Debug)]
pub enum Move {
    Left,
    Right,
    None,
}