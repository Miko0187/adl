pub fn lexer(data: &String) {
    let mut pos = 0;

    while pos < data.len() {
        let c = data.chars().nth(pos).unwrap();

        if c.is_whitespace() {
            pos += 1;
            continue;
        }

        if c.is_ascii_alphabetic() {
            let string = get_string(data, pos);
            pos += string.len();
            println!("STRING: {}", string);
            continue;
        }

        if c.is_ascii_digit() {
            let number = get_number(data, pos);
            pos += number.len();
            println!("NUMBER: {}", number);
            continue;
        }

        match c {
            '+' => println!("PLUS"),
            '-' => println!("MINUS"),
            '*' => println!("STAR"),
            '/' => println!("SLASH"),
            '(' => println!("LPAREN"),
            ')' => println!("RPAREN"),
            '=' => println!("EQUAL"),
            ';' => println!("SEMICOLON"),
            _ => println!("UNKNOWN"),
        }

        pos += 1;
    }
}

fn get_string(data: &String, pos: usize) -> String {
    let mut string = String::new();
    let mut pos = pos;

    while pos < data.len() {
        let c = data.chars().nth(pos).unwrap();
        if !c.is_ascii_alphabetic() {
            break;
        }
        string.push(c);
        pos += 1;
    }

    string
}

fn get_number(data: &String, pos: usize) -> String {
    let mut number = String::new();
    let mut pos = pos;

    while pos < data.len() {
        let c = data.chars().nth(pos).unwrap();
        if !c.is_ascii_digit() {
            break;
        }
        number.push(c);
        pos += 1;
    }

    number
}
