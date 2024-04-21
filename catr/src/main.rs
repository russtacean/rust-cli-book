use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// The file(s) to open, uses STDIN if none supplied
    #[arg(default_value="-")]
    files: Vec<String>,
    /// Number each line
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,
    /// Number non blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn run(args: Args) -> Result<()> {
    for filename in &args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => readFile(file, &args)?
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn readFile(file: Box<dyn BufRead>, args: &Args) -> Result<()>{
    let mut line_num = 0;
    for line_result in file.lines() {
        let line = line_result?;
        line_num += 1;

        if line == "" && args.number_nonblank_lines {
            line_num -= 1;
            println!();
            continue
        }

        if args.number_lines || args.number_nonblank_lines{
            println!("{line_num:>6}\t{line}")
        } else {
            println!("{line}")
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }

}
