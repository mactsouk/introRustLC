use std::env;

pub fn check_palindrome(input: &str) -> bool {
    if input.len() == 0 {
        return true;
    }
    let mut last = input.len() - 1;
    let mut first = 0;
    
    let my_vec = input.as_bytes().to_owned();
    
    while first < last {
        if my_vec[first] != my_vec[last] {
            return false;
        }
        
        first +=1;
        last -=1;
    }
    return true;
}

fn main() {
    let args: Vec<_> = env::args().collect();
	if args.len() == 1 {
 		println!("Usage: {} string", args[0]);
        return;
 	}
    
    let input_string = ::std::env::args().nth(1).unwrap();
    if check_palindrome(&input_string) {
        println!("{} is a palindrome!", input_string);
    } else {
        println!("{} is not a palindrome!", input_string);
    }
}
