use std::io::Write;
use std::io::{stdin, stdout};

#[derive(Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Nul,
}

#[derive(Debug)]
enum Token {
    Operation(Operator),
    Number(f64),
}

impl Token {
    pub fn is_numeric(v: &str) -> bool {
        v.chars().next().unwrap().is_numeric()
    }

    pub fn is_operator(v: &str) -> bool {
        if v.len() > 1 {
            return false;
        }

        match v.chars().next().unwrap() {
            '+' | '-' | '*' | '/' => return true,
            _ => return false,
        }
    }
}

fn prompt(s: &str) {
    print!("{s}");
    stdout().flush().unwrap();
}

fn input() -> String {
    let mut buf: String = String::new();

    stdin().read_line(&mut buf).unwrap();

    buf.trim().to_string()
}

fn is_token_valid(s: &str) -> bool {
    Token::is_numeric(s) || Token::is_operator(s)
}

fn tokenize(expr: String) -> Result<Vec<Token>, String> {
    let mut stack: Vec<Token> = Vec::new();

    let mut last_tok = None;
    for (i, tok) in expr.split(' ').enumerate() {
        if !is_token_valid(&tok) {
            return Err(format!("Token '{tok}' at column '{i}' is invalid"));
        }

        if Token::is_numeric(&tok) {
            let val: f64 = tok.parse().unwrap();

            stack.push(Token::Number(val));
        } else if Token::is_operator(&tok) {
            let current = tok.chars().next();

            let op = match current.unwrap() {
                '+' => Operator::Add,
                '-' => Operator::Sub,
                '*' => Operator::Mul,
                '/' => Operator::Div,
                _ => Operator::Nul,
            };

            match last_tok {
                Some(t) => {
                    if Token::is_operator(t) {
                        return Err(format!(
                            "Syntax error: The operator needs at least one operand"
                        ));
                    }
                }
                None => {}
            }

            stack.push(Token::Operation(op));
        }

        last_tok = Some(tok);
    }
    Ok(stack)
}

fn evaluate(tokens: &Vec<Token>) -> f64 {
    let mut ret: f64 = 0.0;
    let mut stack: Vec<f64> = Vec::new();
    let mut first_val = false;

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Operation(op) => {
                for value in stack.iter() {
                    if !first_val {
                        first_val = true;
                        ret = *value;
                        continue;
                    }

                    ret = match op {
                        Operator::Add => ret + value,
                        Operator::Sub => ret - value,
                        Operator::Div => ret / value,
                        Operator::Mul => ret * value,
                        _ => 0.0,
                    }
                }

                stack = Vec::new();
            }
        }
    }

    ret
}

fn main() {
    loop {
        prompt("> ");
        let expr = input();

        if expr.trim().eq(".exit") {
            break;
        } else if expr.len() == 0 {
            continue;
        } else if expr.starts_with("--") {
            continue;
        }

        let toks = match tokenize(expr) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Syntax error: {e}");
                continue;
            }
        };

        let result = evaluate(&toks);

        println!("{result}");
    }
}
