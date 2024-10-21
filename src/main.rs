use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::turing::strict_printer::{compute_strict_path, def_to_strict_format};
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

    #[arg(short, long, default_value = "false")]
    /// Write the input format (which doesn't follow the TM format strictly) to the strict TM format
    write_transform: bool,
}

fn main() -> anyhow::Result<()> {
    // Get the args
    let args = Args::parse();
    // Read the definition file
    let def_string = fs::read_to_string(&args.tm_def)?;
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
        let mut runner = TuringRunner::new(&def, args.small_output);
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
    // If we need to print the strict format, do the transform
    if args.write_transform {
        // Erzeuge den String-Inhalt mit der Funktion def_to_strict_format
        let content = def_to_strict_format(&def);
        // Öffne die Datei (wird erstellt oder überschrieben, falls vorhanden)
        let strict_path = compute_strict_path(&args.tm_def);
        let path = Path::new(&strict_path);
        let mut file = File::create(&path)?;
        // Schreibe den Inhalt in die Datei
        file.write_all(content.as_bytes())?;
        // Optional: Stelle sicher, dass die Datei geflusht wird
        file.flush()?;
    }
    Ok(())
}
