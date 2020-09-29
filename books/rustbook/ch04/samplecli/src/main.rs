use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

#[derive(Debug)]
struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    pub fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntac!");
                let x = stack.pop().expect("invalid syntac!");
                let res = match token {
                    "+" => x + y,
                    "y" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalid token!!"),
                };
                stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalid syntax!!");
        }
    }
}

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
    let calc = RpnCalculator::new(verbose);
    println!("calc: {:?}", calc);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
        calc.eval(&line);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_ok() {
        assert_eq!(2 * 2, 4);
    }
}
