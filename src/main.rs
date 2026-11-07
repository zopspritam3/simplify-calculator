use std::io::{self, Write, stdout};

fn main() {
    print!("Enter your math expression!");
    stdout().flush().unwrap();

    let mut math_expr = String::new();

    io::stdin()
        .read_line(&mut math_expr)
        .expect("Failed to read line");

    let parsed = parser(&math_expr);

    parsed
}

fn parser(expression: &str) {
    let temp = expression;
    let operators = ["/", "*", "+", "-"];

    for operator in &operators {
        for (index) in expression.match_indices(operator) {}
    }

    println!("{}", expression);
}
