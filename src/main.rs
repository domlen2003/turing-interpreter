use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use log::warn;

use runner::TuringRunner;

use crate::def::TuringDef;

mod parse;
mod def;
mod verify;
mod runner;

#[derive(clap::Parser, Debug)]
#[clap(author = "Dominik Lenz", version, about)]
/// A simple Turing Machine Interpreter
struct Args {
    #[arg(value_name = "TM-FILE")]
    tm_def: PathBuf,
    #[arg(short, long, value_name = "INPUT-TAPE")]
    input: Option<String>,
}


fn main() -> anyhow::Result<()> {
    // Get the args
    let args = Args::parse();
    // Read the definition file
    let def_string = fs::read_to_string(args.tm_def)?;
    // Parse the definition
    let def = TuringDef::parse(&def_string)?;
    // Verify the definition
    def.verify()?;
    println!("Verified definition: {}.", def);
    // If an input tape was provided, run the machine
    if let Some(input) = args.input {
        println!("Running machine with input tape: {}.", input);
        // Build a runner from the definition
        let mut runner: TuringRunner = def.into();
        // Load the tape and print initial state
        runner.load_tape(&input);
        println!("...{}...", runner);
        // Run the machine
        while !runner.is_halted() {
            runner.step();
            println!("...{}...", runner)
        }
    } else {
        warn!("No input tape provided, skipping execution.")
    }
    Ok(())
}
