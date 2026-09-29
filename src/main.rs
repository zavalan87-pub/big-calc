use std::env;
use std::io::{self, BufRead, IsTerminal, Write};
use std::process;

type Number = fn(&str) -> Option<u32>;
type Binary = fn(u32, u32) -> Option<&'static str>;
type Unary = fn(u32) -> Option<&'static str>;

const HOW_TO: &str = "Type something like 7 + 5, 9^2 or sqrt 81";
const ADD_RANGE: &str = "Use whole numbers from 0 to 5759 with +";
const SUBTRACT_RANGE: &str = "Use whole numbers from 0 to 5744 with -";
const MULTIPLY_RANGE: &str = "Use whole numbers from 0 to 5419 with *";
const DIVIDE_RANGE: &str = "Use whole numbers from 0 to 4999 with /";
const SQUARE_RANGE: &str = "Use whole numbers from 0 to 20868924 with ^2";
const SQUARE_ROOT_RANGE: &str = "Use whole numbers from 0 to 20576518 with sqrt";

const NUMBERS: [Number; 7] = [
    numbers_1::number,
    numbers_2::number,
    numbers_3::number,
    numbers_4::number,
    numbers_5::number,
    numbers_6::number,
    numbers_7::number,
];

const ADD: [Binary; 8] = [
    add_1::answer,
    add_2::answer,
    add_3::answer,
    add_4::answer,
    add_5::answer,
    add_6::answer,
    add_7::answer,
    add_8::answer,
];

const SUBTRACT: [Binary; 8] = [
    subtract_1::answer,
    subtract_2::answer,
    subtract_3::answer,
    subtract_4::answer,
    subtract_5::answer,
    subtract_6::answer,
    subtract_7::answer,
    subtract_8::answer,
];

const MULTIPLY: [Binary; 8] = [
    multiply_1::answer,
    multiply_2::answer,
    multiply_3::answer,
    multiply_4::answer,
    multiply_5::answer,
    multiply_6::answer,
    multiply_7::answer,
    multiply_8::answer,
];

const DIVIDE: [Binary; 8] = [
    divide_1::answer,
    divide_2::answer,
    divide_3::answer,
    divide_4::answer,
    divide_5::answer,
    divide_6::answer,
    divide_7::answer,
    divide_8::answer,
];

const SQUARE: [Unary; 8] = [
    square_1::answer,
    square_2::answer,
    square_3::answer,
    square_4::answer,
    square_5::answer,
    square_6::answer,
    square_7::answer,
    square_8::answer,
];

const SQUARE_ROOT: [Unary; 8] = [
    square_root_1::answer,
    square_root_2::answer,
    square_root_3::answer,
    square_root_4::answer,
    square_root_5::answer,
    square_root_6::answer,
    square_root_7::answer,
    square_root_8::answer,
];

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
        | [Token::Number(n), Token::Op('²')] => unary(n, &SQUARE, SQUARE_RANGE),
        [Token::Op('√'), Token::Number(n)] => unary(n, &SQUARE_ROOT, SQUARE_ROOT_RANGE),
        [Token::Number(a), Token::Op(op), Token::Number(b)] => binary(a, op, b),
        _ => Err(HOW_TO),
    }
}

fn number(word: &str) -> Option<u32> {
    NUMBERS.iter().find_map(|part| part(word))
}

fn unary(n: &str, parts: &[Unary], range: &'static str) -> Result<&'static str, &'static str> {
    let n = number(n).ok_or(range)?;
    parts.iter().find_map(|part| part(n)).ok_or(range)
}

fn binary(a: &str, op: char, b: &str) -> Result<&'static str, &'static str> {
    let (parts, range): (&[Binary], &'static str) = match op {
        '+' => (&ADD, ADD_RANGE),
        '-' => (&SUBTRACT, SUBTRACT_RANGE),
        '*' => (&MULTIPLY, MULTIPLY_RANGE),
        '/' => (&DIVIDE, DIVIDE_RANGE),
        _ => return Err(HOW_TO),
    };
    let (Some(a), Some(b)) = (number(a), number(b)) else {
        return Err(range);
    };
    parts.iter().find_map(|part| part(a, b)).ok_or(range)
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
