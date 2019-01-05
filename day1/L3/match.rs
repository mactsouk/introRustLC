use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn main() {	
	let mut total_lines = 0;
    let mut matched_lines = 0;    
    let args: Vec<_> = env::args().collect();

	if args.len() != 3 	{
 		println!("{} filename string", args[0]);
        return;
 	}
 
	let input_path = ::std::env::args().nth(1).unwrap();
	let string_to_match = ::std::env::args().nth(2).unwrap();
	
	let file = BufReader::new(File::open(&input_path).unwrap());
	for line in file.lines() {
		total_lines += 1;
		let my_line = line.unwrap();
		
		if my_line.contains(&string_to_match) {
			println!("{}", my_line);
            matched_lines += 1;
		}
	}

	println!("Lines processed: {}", total_lines);
	println!("Lines matched: {}", matched_lines);
}

