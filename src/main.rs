use std::env;
use std::io::{self, BufRead, IsTerminal, Write};
use std::process;

type Binary = fn(u32, u32) -> Option<&'static str>;
type Unary = fn(u32) -> Option<&'static str>;

const HOW_TO: &str = "Type something like 7 + 5, 9^2 or sqrt 81";

enum Token<'a> {
    Number(&'a str),
    Op(char),
}

// this is the function that does the thing
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if !args.is_empty() {
        match solve(&args.join(" ")) {
            Ok(result) => println!("{result}"),
            Err(message) => {
                eprintln!("{message}");
                process::exit(1);
            }
        }
        return;
    }
    let stdin = io::stdin();
    let interactive = stdin.is_terminal();
    let mut lines = stdin.lock().lines();
    loop {
        if interactive {
            print!("> ");
            let _ = io::stdout().flush();
        }
        let Some(Ok(line)) = lines.next() else {
            if interactive {
                println!();
            }
            break;
        };
        let input = line.trim();
        if input.is_empty() {
            continue;
        }
        if matches!(input, "q" | "quit" | "exit") {
            break;
        }
        match solve(input) {
            Ok(result) => println!("{result}"),
            Err(message) => println!("{message}"),
        }
    }
}

fn solve(input: &str) -> Result<&'static str, &'static str> {
    let tokens = tokenize(input).ok_or(HOW_TO)?;
    match tokens[..] {
        [Token::Number(n), Token::Op('^'), Token::Number("2")]
        | [Token::Number(n), Token::Op('²')] => unary(n, square::answer, square::RANGE),
        [Token::Op('√'), Token::Number(n)] => unary(n, square_root::answer, square_root::RANGE),
        [Token::Number(a), Token::Op(op), Token::Number(b)] => binary(a, op, b),
        _ => Err(HOW_TO),
    }
}

fn unary(n: &str, answer: Unary, range: &'static str) -> Result<&'static str, &'static str> {
    numbers::number(n).and_then(answer).ok_or(range)
}

fn binary(a: &str, op: char, b: &str) -> Result<&'static str, &'static str> {
    let (answer, range): (Binary, &'static str) = match op {
        '+' => (add::answer, add::RANGE),
        '-' => (subtract::answer, subtract::RANGE),
        '*' => (multiply::answer, multiply::RANGE),
        '/' => (divide::answer, divide::RANGE),
        _ => return Err(HOW_TO),
    };
    let (Some(a), Some(b)) = (numbers::number(a), numbers::number(b)) else {
        return Err(range);
    };
    answer(a, b).ok_or(range)
}

fn tokenize(input: &str) -> Option<Vec<Token<'_>>> {
    let mut tokens = Vec::new();
    let mut rest = input.trim_start();
    while let Some(c) = rest.chars().next() {
        if c.is_ascii_digit() {
            let end = rest
                .find(|ch: char| !ch.is_ascii_digit())
                .unwrap_or(rest.len());
            tokens.push(Token::Number(&rest[..end]));
            rest = &rest[end..];
        } else if let Some(after) = rest.strip_prefix("sqrt") {
            tokens.push(Token::Op('√'));
            rest = after;
        } else {
            let op = match c {
                '+' => '+',
                '-' | '−' => '-',
                '*' | 'x' | 'X' | '×' => '*',
                '/' | '÷' => '/',
                '^' => '^',
                '²' => '²',
                '√' => '√',
                _ => return None,
            };
            tokens.push(Token::Op(op));
            rest = &rest[c.len_utf8()..];
        }
        rest = rest.trim_start();
    }
    Some(tokens)
}
