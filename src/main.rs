use std::{
    env, fs,
    path::{Path, PathBuf},
};
extern crate clap;
use clap::{Arg, ArgMatches, Parser};

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
    let args = env::args().collect::<Vec<String>>();

    let file = &args[1];
    wc(&file)
}

fn wc(file: &String) {
    println!(
        "       {}      {}     {} {}",
        count_lines(&file),
        count_words(&file),
        count_chars(&file),
        file
    )
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
