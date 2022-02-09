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
    selected: String,

    /// The prefix template, {} is replaced by selected
    #[clap(short, long, default_value = "$>>")]
    separator: String,

    /// The input file, or stdin if not specified
    #[clap(short, long)]
    input: Option<String>,

    /// The output file, or stdout if not specified
    #[clap(short, long)]
    output: Option<String>,

    #[clap(short, long, requires("input"), requires("output"))]
    watch : bool,
}

fn main() {
    let args = Args::parse();
    if args.watch {
        if args.input.is_none() || args.output.is_none() {
            panic!("watch mode requires input and output files");
        }

        watch_input(&args);

    } else {
        process_input(&args);
    }
}

fn watch_input(args :&Args) {
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();
    watcher.watch(args.input.as_ref().unwrap(), RecursiveMode::NonRecursive).unwrap();
    loop {
        match rx.recv() {
            Ok(RawEvent{path: Some(_), op: Ok(_), cookie: _}) => {
                process_input(args);
            },
            Ok(event) => eprintln!("broken event: {:?}", event),
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}

fn process_input(args :&Args) {
    let selected = args.selected.to_string() + &args.separator;
    let reader: Box<dyn BufRead> = match &args.input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap())),
    };
    let mut writer: Box<dyn Write> = match &args.output {
        None => Box::new(BufWriter::new(io::stdout())),
        Some(filename) => Box::new(BufWriter::new(File::create(filename).unwrap())),
    };
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&args.separator) {
            if line.starts_with(selected.as_str()) {
                writeln!(&mut writer, "{}", &line[selected.len()..]).unwrap();
            }
        } else {
            writeln!(&mut writer, "{}", line).unwrap();
        }
    }
}