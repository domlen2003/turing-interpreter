use std::fs;
use std::path::PathBuf;

use clap::Parser;

use turing::TuringDef;

mod turing;

#[derive(Parser, Debug)]
#[clap(author = "Dominik Lenz", version, about)]
/// A simple Turing Machine Interpreter
struct Args {
    #[arg(value_name = "TM-FILE")]
    tm_def: PathBuf,
}

fn main() {
    // Get the args and file
    let args = Args::parse();
    let def_string = fs::read_to_string(args.tm_def).expect("Could not read the TM definition file");
    // Parse the definition
    let def = TuringDef::parse(&def_string).expect("Could not parse the TM definition");
    // Verify the definition
    def.verify().expect("Could not verify the TM definition");
    println!("{:?}", def);
}
