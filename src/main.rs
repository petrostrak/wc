use std::{env, fs};

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
