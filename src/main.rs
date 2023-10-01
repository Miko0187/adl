mod lexer;
pub mod token;

use std::io::{self, Write};

fn main() {
    println!("A Data Language");
    println!("Not meant for actual use");

    loop {
        print!("-> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to readline");

        let tokens = lexer::lexer(&input);

        for token in tokens {
            println!("{}", token);
        }
    }
}
