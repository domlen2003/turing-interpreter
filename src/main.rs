use std::fs;
use std::path::PathBuf;
use clap::Parser;
use crate::types::TuringDef;

mod parse;
mod types;
mod verify;
mod runner;

#[derive(clap::Parser, Debug)]
#[clap(author = "Dominik Lenz", version, about)]
/// A simple Turing Machine Interpreter
struct Args {
    #[arg(value_name = "TM-FILE")]
    tm_def: PathBuf,
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
    // Run the machine
    runner::run(def)?;
    Ok(())
}
