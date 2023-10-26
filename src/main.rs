use std::fs;
use std::path::PathBuf;

use clap::Parser;

use turing::def::TuringDef;
use turing::runner::TuringRunner;

mod turing;

#[derive(clap::Parser, Debug)]
#[clap(author = "Dominik Lenz", version, about)]
/// A simple Turing Machine Interpreter
struct Args {
    #[arg(value_name = "TM-FILE")]
    /// The Turing Machine definition file to use
    tm_def: PathBuf,
    #[arg(short, long)]
    /// The input tape to run the machine with
    input: Option<String>,
    #[arg(short, long, default_value = "false")]
    /// Print the definition after parsing
    verbose: bool,
    #[arg(short, long, default_value = "false")]
    /// Use a smaller output format when printing the tape
    small_output: bool,
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
    if args.verbose {
        println!("Verified definition:\n{}", def);
    }
    // If an input tape was provided, run the machine
    if let Some(input) = args.input {
        if args.verbose {
            println!("Running machine with input tape: {}.", input);
        }
        // Build a runner from the definition
        let mut runner = TuringRunner::new(def, args.small_output);
        // Load the tape and print initial state
        runner.load_tape(&input);
        println!("...{}...", runner);
        // Run the machine
        while !runner.is_halted() {
            runner.step();
            println!("...{}...", runner);
        }
    } else {
        println!("No input tape provided, skipping execution.")
    }
    Ok(())
}
