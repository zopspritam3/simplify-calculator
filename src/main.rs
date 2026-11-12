use std::{
    io::{self, Write, stdout},
    iter::Peekable,
};

fn main() {
    print!("Enter your math expression!");
    stdout().flush().unwrap();

    let mut math_expr = String::new();

    io::stdin()
        .read_line(&mut math_expr)
        .expect("Failed to read line");

    let parsed = parser(&math_expr);
}

fn parser(expression: &str) -> Result<i64, string> {
    let chars = expression.chars().peekable();

    let parsed_expr = parse_expression(chars);

    if chars.peek().is_some() {
        return Err("Invalid expression".to_string());
    }

    parsed_expr
}

fn parse_exprsn<I>(expression: &mut std::iter::Peekable<I>) -> Result<i64, string>
where
    I: Iterator<Item = char>,
{
    let mut left = parse_item(expression);

    while let Some(&op) = expression.peek() {
        if op != '+' || op != '-' {
            break;
        }
        let mut right = parse_item(expression);
        retu
    }
}

fn parse_groups<I>(expression: &mut std::iter::Peekable<I>) -> Result<i64, string>
where
    I: Iterator<Item = char>,
{
    match expression.peek() {
        Some('(') => {
            expression.next();
            let under_parantheses = parse_expression(expression)?;
            if expression.next() != Some(')') {
                return Err("No closing bracket found".to_string());
            }

            Ok(under_parantheses)
        }

        Some(c) if c.is_ascii_digit() => {
            let mut num = 0;
            while let Some(&c) = expression.peek() {
                if !c.is_ascii_digit() {
                    break;
                }
                num = num * 10 + (c as i64 - '0' as i64);
                expression.next();
            }

            Ok(num)
        }

        _ => Err("Invalid expression".to_string()),
    }
}
