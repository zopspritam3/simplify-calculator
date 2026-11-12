use std::io::{self, Write, stdout};

fn main() {
    print!("Enter your math expression!");
    stdout().flush().unwrap();

    let mut math_expr = String::new();

    io::stdin()
        .read_line(&mut math_expr)
        .expect("Failed to read line");

    let parsed = parser(&math_expr);

    match parsed {
        Ok(a) => println!("{:?} is the final answer", a),
        Err(msg) => print!("{}", msg),
    }
}

fn parser(expression: &str) -> Result<i64, String> {
    let mut chars = expression.chars().filter(|c| !c.is_whitespace()).peekable();

    let parsed_expr = parse_exprsn(&mut chars)?;

    if chars.peek().is_some() {
        return Err("Invalid expression".into());
    }

    Ok(parsed_expr)
}

fn parse_exprsn<I>(expression: &mut std::iter::Peekable<I>) -> Result<i64, String>
where
    I: Iterator<Item = char>,
{
    let mut left = parse_item(expression)?;

    while let Some(&op) = expression.peek() {
        if op != '+' && op != '-' {
            break;
        }
        expression.next();
        let right = parse_item(expression)?;
        left = if op == '+' {
            left + right
        } else {
            left - right
        };
    }

    Ok(left)
}

fn parse_groups<I>(expression: &mut std::iter::Peekable<I>) -> Result<i64, String>
where
    I: Iterator<Item = char>,
{
    match expression.peek() {
        Some('(') => {
            expression.next();
            let under_parantheses = parse_exprsn(expression)?;
            if expression.next() != Some(')') {
                return Err("No closing bracket found".into());
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

        _ => Err("Invalid expression".into()),
    }
}

fn parse_item<I>(expression: &mut std::iter::Peekable<I>) -> Result<i64, String>
where
    I: Iterator<Item = char>,
{
    let mut left = parse_groups(expression)?;

    while let Some(&op) = expression.peek() {
        if op != '/' && op != '*' {
            break;
        }
        expression.next();
        let right = parse_groups(expression)?;
        left = if op == '*' {
            left * right
        } else {
            left / right
        };
    }

    Ok(left)
}
