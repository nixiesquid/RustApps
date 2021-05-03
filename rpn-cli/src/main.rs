use clap::Clap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clap, Debug)]
#[clap(name = "RPN program", version = "1.0.0", about = "RPN Calculator")]
struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,
    /// Formulas written in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        run(reader, opts.verbose);
    } else {
        println!("No file is specified.");
    }
}

fn run(reader: BufReader<File>, verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }

    let _ = verbose;
}
