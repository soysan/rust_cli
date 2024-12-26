use clap::Parser;
use std::{fs, path, io};
use std::io::BufRead;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'n', long)]
    name: String,
    #[arg(short = 'a', long, default_value = "42")]
    age: u32,
    #[arg(short, long)]
    path: path::PathBuf,
}

fn main() {
    let args = Args::parse();
    let content = fs::File::open(&args.path).expect("file not found");
    let reader = io::BufReader::new(content);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&args.name) {
            println!("{} is {} years old", args.name, args.age);
        }
    }
}

