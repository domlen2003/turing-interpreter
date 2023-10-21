use thiserror::Error;

use crate::parse::TuringParseError::InvalidTransitionFunction;
use crate::parse::TuringTransitionError::{InvalidArgumentCount, InvalidInput, InvalidMove, InvalidNextState, InvalidState};
use crate::types::{Move, TransitionFunction};

#[derive(Error, Debug)]
pub enum TuringParseError {
    #[error("Could not parse the state count")]
    InvalidStateCount,
    #[error("Could not parse the alphabet")]
    InvalidInputAlphabet,
    #[error("Could not parse the tape alphabet")]
    InvalidTapeAlphabet,
    #[error("Could not parse the start state")]
    InvalidStartState,
    #[error("Could not parse the end state")]
    InvalidEndState,
    #[error("Could not parse transition function {0}: {1}")]
    InvalidTransitionFunction(usize, TuringTransitionError),
    #[error("Invalid argument count")]
    InvalidArgumentCount,
}

#[derive(Debug, Error)]
pub enum TuringTransitionError {
    #[error("Invalid argument count")]
    InvalidArgumentCount,
    #[error("Invalid state")]
    InvalidState,
    #[error("Invalid input")]
    InvalidInput,
    #[error("Invalid next state")]
    InvalidNextState,
    #[error("Invalid move")]
    InvalidMove,
}


impl crate::types::TuringDef {
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
            .iter().enumerate()
            .map(|(pos, &line)| TransitionFunction::parse(pos, line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(crate::types::TuringDef {
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
    fn parse(pos: usize, line: &str) -> Result<Self, TuringParseError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 5 {
            return Err(InvalidTransitionFunction(pos, InvalidArgumentCount));
        }

        let state = parts[0].parse().map_err(|_| InvalidTransitionFunction(pos, InvalidState))?;
        let input = parts[1].chars().next().ok_or(InvalidTransitionFunction(pos, InvalidInput))?;
        let next_state = parts[2].parse().map_err(|_| InvalidTransitionFunction(pos, InvalidNextState))?;
        let write = parts[3].chars().next().ok_or(InvalidTransitionFunction(pos, InvalidMove))?;
        let move_dir = match parts[4] {
            "L" => Move::Left,
            "R" => Move::Right,
            "N" => Move::None,
            _ => return Err(InvalidTransitionFunction(pos, InvalidMove)),
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