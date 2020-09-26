use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "me",
    about = "Super awesome sample RPN calculator"
)]

struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Fomulas writtend in RPN
    #[clap(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        // TODO: エラーハンドリング
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, false);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
        println!("No file is specified!");
    }
}

fn run<R: BufRead>(reader: R, verbose: bool) {
    println!("Is verbosity?: {}", verbose);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}
