use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'n', long)]
    name: String,
    #[arg(short = 'a', long, default_value = "42")]
    age: u32,
}

fn main() {
    let args = Args::parse();

    println!("name: '{:?}', age: '{:?}'", args.name, args.age);
}

