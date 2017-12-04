use std::io;
use std::iter::FromIterator;

struct LineReader;

fn is_valid_line(line: &String, sort: bool) -> bool {
    let word_iter = line.split(" ");
    let mut words = if sort {
        word_iter.map(|val| {
            let mut chars = val.chars().collect::<Vec<char>>();
            chars.sort_by(|a, b| b.cmp(a));
            String::from_iter(chars)
        }).collect::<Vec<String>>()
    } else {
        word_iter.map(|val| val.to_string()).collect::<Vec<String>>()
    };

    let base_count = words.len();
    words.sort();
    words.dedup();

    base_count == words.len()
}

impl Iterator for LineReader {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut val = String::new();
        match io::stdin().read_line(&mut val) {
            Err(_) => None,
            Ok(_) => {
                let trimmed_val = val.trim();

                if trimmed_val.len() > 1 {
                    Some(trimmed_val.to_owned())
                } else {
                    None
                }
            }
        }
    }
}

fn line_reader() -> LineReader {
    LineReader
}

fn main() {
    println!("Please enter the passphrases, followed by an empty line:");

    // set to false for challenge 1, true for challenge 2
    const SORT_LETTERS: bool = true;

    let lines = line_reader();
    let valid_phrases = lines.filter(|x| is_valid_line(x, SORT_LETTERS)).collect::<Vec<String>>();
    println!("There are {} valid phrases", valid_phrases.len());
}
