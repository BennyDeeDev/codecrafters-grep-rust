use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        p if p.starts_with("[") && p.ends_with("]") && p.len() >= 2 => {
            let inner = &p[1..p.len() - 1];
            input_line
                .chars()
                .any(|c1| inner.chars().any(|c2| c1 == c2))
        }
        "\\w" => input_line.chars().any(|c| c.is_alphanumeric() || c == '_'),
        "\\d" => input_line.chars().any(|c| c.is_ascii_digit()),
        _ => input_line.contains(pattern),
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    let result = match_pattern(&input_line, &pattern);

    println!("{}", result);

    if result {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
