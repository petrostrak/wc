use std::fs;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// A command option. Can be:
    /// c for counting the number of bytes
    /// w for counting the number of words
    /// l for counting the number of lines
    /// m for counting the number of characters
    #[arg(short)]
    option: Option<String>,

    /// The file to read
    #[arg(short)]
    file: String,
}

fn main() {
    let args = Cli::parse();

    let file = args.file;

    if let Some(option) = args.option.as_deref() {
        match option {
            "c" => println!("\t{} {}", count_bytes(&file), file),
            "w" => println!("\t{} {}", count_words(&file), file),
            "l" => println!("\t{} {}", count_lines(&file), file),
            "m" => println!("\t{} {}", count_chars(&file), file),
            _ => wc(&file),
        }
    }
}

fn wc(file: &String) {
    println!(
        "       {}      {}     {} {}",
        count_lines(&file),
        count_words(&file),
        count_chars(&file),
        file
    );
}

fn count_bytes(file: &String) -> usize {
    fs::read_to_string(file)
        .expect("cannot read file")
        .as_bytes()
        .len()
}

fn count_lines(file: &String) -> usize {
    fs::read_to_string(file)
        .expect("cannot read file")
        .lines()
        .count()
}

fn count_words(file: &String) -> usize {
    let mut word_count = 0;
    fs::read_to_string(file)
        .expect("cannot read file")
        .lines()
        .for_each(|l| word_count += l.split_ascii_whitespace().count());

    word_count
}

fn count_chars(file: &String) -> usize {
    fs::read_to_string(file)
        .expect("cannot read file")
        .chars()
        .count()
}
