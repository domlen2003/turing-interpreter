use thiserror::Error;

use crate::parse::TuringParseError::InvalidTransitionFunction;
use crate::parse::TuringTransitionError::{InvalidArgumentCount, InvalidInput, InvalidMove, InvalidNextState, InvalidState};
use crate::def::{Move, TransitionFunction};

#[derive(Error, Debug)]
pub enum TuringParseError {
    #[error("Could not parse the state count")]
    InvalidStateCount,
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


impl crate::def::TuringDef {
    /**
     *Parses a Turing Machine Definition from a string
     */
    pub fn parse(value: &str) -> Result<Self, TuringParseError> {
        let lines = value.lines().collect::<Vec<_>>();

        if lines.len() < 5 {
            return Err(TuringParseError::InvalidArgumentCount);
        }

        let state_count = lines[0].parse().map_err(|_| TuringParseError::InvalidStateCount)?;
        let input_alphabet = lines[1].chars().collect();
        let tape_alphabet = lines[2].chars().collect();
        let start_state = lines[3].parse().map_err(|_| TuringParseError::InvalidStartState)?;
        let end_state = lines[4].parse().map_err(|_| TuringParseError::InvalidEndState)?;
        let transition_function = lines[5..].iter().enumerate()
            .map(|(pos, &line)| TransitionFunction::parse(pos, line))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(crate::def::TuringDef {
            state_count,
            input_alphabet,
            tape_alphabet,
            start_state,
            end_state,
            transition_function,
        })
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