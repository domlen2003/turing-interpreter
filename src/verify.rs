use thiserror::Error;

use crate::types::TuringDef;

#[derive(Error, Debug)]
#[error("Invalid transition function: State q{0} does not have a transition for input {1}")]
pub struct TuringVerifyError(usize, char);


impl TuringDef {
    /**
     *Verifies the Turing Machine Definition to be valid (and consistent with itself)
     */
    pub fn verify(&self) -> Result<(), TuringVerifyError> {
        for state in 1..self.state_count as usize {
            for input_char in &self.tape_alphabet {
                self.transition_function.iter()
                    .find(|&x| x.state == state as u8 && x.input == *input_char)
                    .ok_or(TuringVerifyError(state, *input_char))?;
            }
        }
        Ok(())
    }
}