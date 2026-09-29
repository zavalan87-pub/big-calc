use std::env;
use std::io::{self, BufRead, IsTerminal, Write};
use std::process;

type Number = fn(&str) -> Option<u32>;
type Binary = fn(u32, u32) -> Option<&'static str>;
type Unary = fn(u32) -> Option<&'static str>;

const HOW_TO: &str = "Type something like 7 + 5, 9^2 or sqrt 81";
const ADD_RANGE: &str = "Use whole numbers from 0 to 12623 with +";
const SUBTRACT_RANGE: &str = "Use whole numbers from 0 to 12699 with -";
const MULTIPLY_RANGE: &str = "Use whole numbers from 0 to 11919 with *";
const DIVIDE_RANGE: &str = "Use whole numbers from 0 to 11129 with /";
const SQUARE_RANGE: &str = "Use whole numbers from 0 to 99980027 with ^2";
const SQUARE_ROOT_RANGE: &str = "Use whole numbers from 0 to 101552860 with sqrt";

const NUMBERS: [Number; 34] = [
    numbers_1::part_01::number,
    numbers_1::part_02::number,
    numbers_1::part_03::number,
    numbers_1::part_04::number,
    numbers_1::part_05::number,
    numbers_1::part_06::number,
    numbers_1::part_07::number,
    numbers_1::part_08::number,
    numbers_1::part_09::number,
    numbers_1::part_10::number,
    numbers_2::part_01::number,
    numbers_2::part_02::number,
    numbers_2::part_03::number,
    numbers_2::part_04::number,
    numbers_2::part_05::number,
    numbers_2::part_06::number,
    numbers_2::part_07::number,
    numbers_2::part_08::number,
    numbers_2::part_09::number,
    numbers_2::part_10::number,
    numbers_3::part_01::number,
    numbers_3::part_02::number,
    numbers_3::part_03::number,
    numbers_3::part_04::number,
    numbers_3::part_05::number,
    numbers_3::part_06::number,
    numbers_3::part_07::number,
    numbers_3::part_08::number,
    numbers_3::part_09::number,
    numbers_3::part_10::number,
    numbers_4::part_01::number,
    numbers_4::part_02::number,
    numbers_4::part_03::number,
    numbers_4::part_04::number,
];

const ADD: [Binary; 40] = [
    add_1::part_01::answer,
    add_1::part_02::answer,
    add_1::part_03::answer,
    add_1::part_04::answer,
    add_1::part_05::answer,
    add_1::part_06::answer,
    add_1::part_07::answer,
    add_1::part_08::answer,
    add_1::part_09::answer,
    add_1::part_10::answer,
    add_2::part_01::answer,
    add_2::part_02::answer,
    add_2::part_03::answer,
    add_2::part_04::answer,
    add_2::part_05::answer,
    add_2::part_06::answer,
    add_2::part_07::answer,
    add_2::part_08::answer,
    add_2::part_09::answer,
    add_2::part_10::answer,
    add_3::part_01::answer,
    add_3::part_02::answer,
    add_3::part_03::answer,
    add_3::part_04::answer,
    add_3::part_05::answer,
    add_3::part_06::answer,
    add_3::part_07::answer,
    add_3::part_08::answer,
    add_3::part_09::answer,
    add_3::part_10::answer,
    add_4::part_01::answer,
    add_4::part_02::answer,
    add_4::part_03::answer,
    add_4::part_04::answer,
    add_4::part_05::answer,
    add_4::part_06::answer,
    add_4::part_07::answer,
    add_4::part_08::answer,
    add_4::part_09::answer,
    add_4::part_10::answer,
];

const SUBTRACT: [Binary; 40] = [
    subtract_1::part_01::answer,
    subtract_1::part_02::answer,
    subtract_1::part_03::answer,
    subtract_1::part_04::answer,
    subtract_1::part_05::answer,
    subtract_1::part_06::answer,
    subtract_1::part_07::answer,
    subtract_1::part_08::answer,
    subtract_1::part_09::answer,
    subtract_1::part_10::answer,
    subtract_2::part_01::answer,
    subtract_2::part_02::answer,
    subtract_2::part_03::answer,
    subtract_2::part_04::answer,
    subtract_2::part_05::answer,
    subtract_2::part_06::answer,
    subtract_2::part_07::answer,
    subtract_2::part_08::answer,
    subtract_2::part_09::answer,
    subtract_2::part_10::answer,
    subtract_3::part_01::answer,
    subtract_3::part_02::answer,
    subtract_3::part_03::answer,
    subtract_3::part_04::answer,
    subtract_3::part_05::answer,
    subtract_3::part_06::answer,
    subtract_3::part_07::answer,
    subtract_3::part_08::answer,
    subtract_3::part_09::answer,
    subtract_3::part_10::answer,
    subtract_4::part_01::answer,
    subtract_4::part_02::answer,
    subtract_4::part_03::answer,
    subtract_4::part_04::answer,
    subtract_4::part_05::answer,
    subtract_4::part_06::answer,
    subtract_4::part_07::answer,
    subtract_4::part_08::answer,
    subtract_4::part_09::answer,
    subtract_4::part_10::answer,
];

const MULTIPLY: [Binary; 40] = [
    multiply_1::part_01::answer,
    multiply_1::part_02::answer,
    multiply_1::part_03::answer,
    multiply_1::part_04::answer,
    multiply_1::part_05::answer,
    multiply_1::part_06::answer,
    multiply_1::part_07::answer,
    multiply_1::part_08::answer,
    multiply_1::part_09::answer,
    multiply_1::part_10::answer,
    multiply_2::part_01::answer,
    multiply_2::part_02::answer,
    multiply_2::part_03::answer,
    multiply_2::part_04::answer,
    multiply_2::part_05::answer,
    multiply_2::part_06::answer,
    multiply_2::part_07::answer,
    multiply_2::part_08::answer,
    multiply_2::part_09::answer,
    multiply_2::part_10::answer,
    multiply_3::part_01::answer,
    multiply_3::part_02::answer,
    multiply_3::part_03::answer,
    multiply_3::part_04::answer,
    multiply_3::part_05::answer,
    multiply_3::part_06::answer,
    multiply_3::part_07::answer,
    multiply_3::part_08::answer,
    multiply_3::part_09::answer,
    multiply_3::part_10::answer,
    multiply_4::part_01::answer,
    multiply_4::part_02::answer,
    multiply_4::part_03::answer,
    multiply_4::part_04::answer,
    multiply_4::part_05::answer,
    multiply_4::part_06::answer,
    multiply_4::part_07::answer,
    multiply_4::part_08::answer,
    multiply_4::part_09::answer,
    multiply_4::part_10::answer,
];

const DIVIDE: [Binary; 40] = [
    divide_1::part_01::answer,
    divide_1::part_02::answer,
    divide_1::part_03::answer,
    divide_1::part_04::answer,
    divide_1::part_05::answer,
    divide_1::part_06::answer,
    divide_1::part_07::answer,
    divide_1::part_08::answer,
    divide_1::part_09::answer,
    divide_1::part_10::answer,
    divide_2::part_01::answer,
    divide_2::part_02::answer,
    divide_2::part_03::answer,
    divide_2::part_04::answer,
    divide_2::part_05::answer,
    divide_2::part_06::answer,
    divide_2::part_07::answer,
    divide_2::part_08::answer,
    divide_2::part_09::answer,
    divide_2::part_10::answer,
    divide_3::part_01::answer,
    divide_3::part_02::answer,
    divide_3::part_03::answer,
    divide_3::part_04::answer,
    divide_3::part_05::answer,
    divide_3::part_06::answer,
    divide_3::part_07::answer,
    divide_3::part_08::answer,
    divide_3::part_09::answer,
    divide_3::part_10::answer,
    divide_4::part_01::answer,
    divide_4::part_02::answer,
    divide_4::part_03::answer,
    divide_4::part_04::answer,
    divide_4::part_05::answer,
    divide_4::part_06::answer,
    divide_4::part_07::answer,
    divide_4::part_08::answer,
    divide_4::part_09::answer,
    divide_4::part_10::answer,
];

const SQUARE: [Unary; 40] = [
    square_1::part_01::answer,
    square_1::part_02::answer,
    square_1::part_03::answer,
    square_1::part_04::answer,
    square_1::part_05::answer,
    square_1::part_06::answer,
    square_1::part_07::answer,
    square_1::part_08::answer,
    square_1::part_09::answer,
    square_1::part_10::answer,
    square_2::part_01::answer,
    square_2::part_02::answer,
    square_2::part_03::answer,
    square_2::part_04::answer,
    square_2::part_05::answer,
    square_2::part_06::answer,
    square_2::part_07::answer,
    square_2::part_08::answer,
    square_2::part_09::answer,
    square_2::part_10::answer,
    square_3::part_01::answer,
    square_3::part_02::answer,
    square_3::part_03::answer,
    square_3::part_04::answer,
    square_3::part_05::answer,
    square_3::part_06::answer,
    square_3::part_07::answer,
    square_3::part_08::answer,
    square_3::part_09::answer,
    square_3::part_10::answer,
    square_4::part_01::answer,
    square_4::part_02::answer,
    square_4::part_03::answer,
    square_4::part_04::answer,
    square_4::part_05::answer,
    square_4::part_06::answer,
    square_4::part_07::answer,
    square_4::part_08::answer,
    square_4::part_09::answer,
    square_4::part_10::answer,
];

const SQUARE_ROOT: [Unary; 40] = [
    square_root_1::part_01::answer,
    square_root_1::part_02::answer,
    square_root_1::part_03::answer,
    square_root_1::part_04::answer,
    square_root_1::part_05::answer,
    square_root_1::part_06::answer,
    square_root_1::part_07::answer,
    square_root_1::part_08::answer,
    square_root_1::part_09::answer,
    square_root_1::part_10::answer,
    square_root_2::part_01::answer,
    square_root_2::part_02::answer,
    square_root_2::part_03::answer,
    square_root_2::part_04::answer,
    square_root_2::part_05::answer,
    square_root_2::part_06::answer,
    square_root_2::part_07::answer,
    square_root_2::part_08::answer,
    square_root_2::part_09::answer,
    square_root_2::part_10::answer,
    square_root_3::part_01::answer,
    square_root_3::part_02::answer,
    square_root_3::part_03::answer,
    square_root_3::part_04::answer,
    square_root_3::part_05::answer,
    square_root_3::part_06::answer,
    square_root_3::part_07::answer,
    square_root_3::part_08::answer,
    square_root_3::part_09::answer,
    square_root_3::part_10::answer,
    square_root_4::part_01::answer,
    square_root_4::part_02::answer,
    square_root_4::part_03::answer,
    square_root_4::part_04::answer,
    square_root_4::part_05::answer,
    square_root_4::part_06::answer,
    square_root_4::part_07::answer,
    square_root_4::part_08::answer,
    square_root_4::part_09::answer,
    square_root_4::part_10::answer,
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
