use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short)]
    count: Option<PathBuf>, // Number of bytes

    #[arg(short)]
    lines: Option<PathBuf>, // Number of lines

    #[arg(short)]
    words: Option<PathBuf>, // Number of words

    #[arg(short)]
    max_chars: Option<PathBuf>, // Maximum number of characters
}

fn main() {
    let args = Args::parse();

    if let Some(file) = args.count.as_deref() {
        println!("{}", count_bytes(file))
    }

    if let Some(file) = args.lines.as_deref() {
        count_lines(file)
    }

    if let Some(file) = args.words.as_deref() {
        count_words(file)
    }

    if let Some(file) = args.max_chars.as_deref() {
        count_chars(file)
    }
}

fn count_bytes(file: &Path) -> usize {
    fs::read_to_string(file).expect("cannot read file").len()
}

fn count_lines(file: &Path) {
    println!("{file:?}")
}

fn count_words(file: &Path) {
    println!("{file:?}")
}

fn count_chars(file: &Path) {
    println!("{file:?}")
}
