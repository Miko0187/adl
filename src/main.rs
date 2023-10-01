mod lexer;
pub mod token;

fn main() {
    println!("A Data Language");
    println!("Not meant for actual use");

    loop {
        println!("> ");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to readline");

        lexer::lexer(&input);
    }
}
