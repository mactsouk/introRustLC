use std::env;

fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        return 0;
    }

    if n <= 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    println!("{:?}", env::args());

    for input in env::args().skip(1) {
        let _i = match input.parse::<i64>() {
          Ok(_i) => {
              println!("Fibonacci number {} is {} ", _i, fibonacci(_i));
          },
          Err(_e) => {
              println!("{}: Not a valid integer!", input)
          }
        };
    }
}
