use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    count: u32, // Number of bytes

    #[arg(short, long)]
    lines: u32, // Number of lines

    #[arg(short, long)]
    words: u32, // Number of words

    #[arg(short, long)]
    max_chars: i32, // Maximum number of characters
}

fn main() {
    let args = Args::parse();

    match args {
        _ => todo!(),
    }
}
