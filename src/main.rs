extern crate notify;

use clap::Parser;
use std::fs::File;
use std::io;

use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use notify::{Watcher, RecursiveMode, raw_watcher, RawEvent};
use std::sync::mpsc::channel;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The selected
    #[clap(short, long)]
    choice: String,

    /// Separates choice from the line chosen
    #[clap(short, long, default_value = "$>>")]
    separator: String,

    /// The input file, or stdin if not specified
    #[clap(short, long)]
    input: Option<String>,

    /// The output file, or stdout if not specified
    #[clap(short, long)]
    output: Option<String>,

    /// Watch the input file for changes and update the output file when it does
    #[clap(short, long, requires("input"), requires("output"))]
    watch : bool,
}

fn main() {
    let args = Args::parse();
    if args.watch {
        if args.input.is_none() || args.output.is_none() {
            panic!("watch mode requires input and output files");
        }

        watch_input(&args.input.unwrap(), &args.output.unwrap(), &args.choice, &args.separator);

    } else {
        process_input(&args.input, &args.output, &args.choice, &args.separator);
    }
}

fn watch_input(input : &String, output : &String, choice : &String, seperator : &String) {
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();
    watcher.watch(input, RecursiveMode::NonRecursive).unwrap();
    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(_path), op: Ok(_), cookie: _}) => {
                process_input(&Option::Some(input.to_owned()), &Option::Some(output.to_owned()), choice, seperator);
            },
            Ok(event) => eprintln!("broken event: {:?}", event),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}

fn process_input(input : &Option<String>, output : &Option<String>, choice : &String, seperator : &String) {
    let choice = choice.to_owned() + seperator;
    let reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).expect("Couldn't read input file"))),
    };
    let mut writer: Box<dyn Write> = match output {
        None => Box::new(BufWriter::new(io::stdout())),
        Some(filename) => Box::new(BufWriter::new(File::create(filename).unwrap())),
    };
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(seperator) {
            if line.starts_with(choice.as_str()) {
                writeln!(&mut writer, "{}", &line[choice.len()..]).unwrap();
            }
        } else {
            writeln!(&mut writer, "{}", line).unwrap();
        }
    }
}