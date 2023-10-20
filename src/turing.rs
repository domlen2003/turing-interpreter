#[derive(Debug)]
pub struct TuringDef {
    // n = #Q = #{q1, q2, ... , qn}
    state_count: u8,
    // Σ
    input_alphabet: Vec<char>,
    // Γ
    tape_alphabet: Vec<char>,
    // q0
    start_state: u8,
    // q^
    end_state: u8,
    // δ
    transition_function: Vec<TransitionFunction>,
}


#[derive(Debug)]
struct TransitionFunction {
    // q
    state: u8,
    // a
    input: char,
    // q'
    next_state: u8,
    // b
    write: char,
    // L, R or N
    move_dir: Move,
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    None,
}

#[derive(Debug)]
pub enum TuringParseError {
    //TODO: add more specific errors/args
    InvalidStateCount,
    InvalidInputAlphabet,
    InvalidTapeAlphabet,
    InvalidStartState,
    InvalidEndState,
    InvalidTransitionFunction,
    InvalidArgumentCount,
}

#[derive(Debug)]
pub enum TuringVerifyError {
}

impl TuringDef {
    /**
     *Verifies the Turing Machine Definition to be valid (and consistent with itself)
     */
    pub fn verify() -> Result<(), TuringVerifyError> {
        todo!("Implement this function")
    }

    /**
     *Parses a Turing Machine Definition from a string
     */
    pub fn parse(value: &str) -> Result<Self, TuringParseError> {
        let lines = value.lines().collect::<Vec<_>>();

        if lines.len() < 5 {
            return Err(TuringParseError::InvalidArgumentCount);
        }

        let state_count = Self::parse_state_count(lines[0])?;
        let input_alphabet = Self::parse_alphabet(lines[1])?;
        let tape_alphabet = Self::parse_alphabet(lines[2])?;
        let start_state = Self::parse_state(lines[3])?;
        let end_state = Self::parse_state(lines[4])?;

        let transition_function = lines[5..]
            .iter()
            .map(|&line| TransitionFunction::parse(line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TuringDef {
            state_count,
            input_alphabet,
            tape_alphabet,
            start_state,
            end_state,
            transition_function,
        })
    }

    fn parse_state_count(line: &str) -> Result<u8, TuringParseError> {
        line.parse().map_err(|_| TuringParseError::InvalidStateCount)
    }

    fn parse_alphabet(line: &str) -> Result<Vec<char>, TuringParseError> {
        Ok(line.chars().collect())
    }

    fn parse_state(line: &str) -> Result<u8, TuringParseError> {
        line.parse().map_err(|_| TuringParseError::InvalidStartState)
    }
}

impl TransitionFunction {
    fn parse(line: &str) -> Result<Self, TuringParseError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 5 {
            return Err(TuringParseError::InvalidTransitionFunction);
        }

        let state = parts[0].parse().map_err(|_| TuringParseError::InvalidTransitionFunction)?;
        let input = parts[1].chars().next().ok_or(TuringParseError::InvalidTransitionFunction)?;
        let next_state = parts[2].parse().map_err(|_| TuringParseError::InvalidTransitionFunction)?;
        let write = parts[3].chars().next().ok_or(TuringParseError::InvalidTransitionFunction)?;
        let move_dir = match parts[4] {
            "L" => Move::Left,
            "R" => Move::Right,
            "N" => Move::None,
            _ => return Err(TuringParseError::InvalidTransitionFunction),
        };

        Ok(TransitionFunction {
            state,
            input,
            next_state,
            write,
            move_dir,
        })
    }
}