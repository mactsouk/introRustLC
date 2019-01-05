use std::env;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() { 
    let mut lines = 0;
    let mut words = 0;
    let mut chars = 0;

    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} text_file(s)", args[0]);
        return;
    }

    let n_args = args.len();
    for x in 1..n_args {
        let mut total_lines = 0;
        let mut total_words = 0;
        let mut total_chars = 0;

        let input_path = ::std::env::args().nth(x).unwrap();
        let file = BufReader::new(File::open(&input_path).unwrap());
        for line in file.lines() {
            let my_line = line.unwrap();
            total_lines = total_lines + 1;
            total_words += my_line.split_whitespace().count();
            total_chars = total_chars + my_line.len() + 1;
        }

        println!("\t{}\t{}\t{}\t{}", total_lines, total_words, total_chars, input_path);
        lines += total_lines;
        words += total_words;
        chars += total_chars;
    }

    if n_args-1 != 1 {
        println!("\t{}\t{}\t{}\ttotal", lines, words, chars);
    }
}

