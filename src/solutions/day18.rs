use super::prelude::*;
use crate::util;

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Value(u64),
    Add,
    Mul,
    LParen,
    RParen,
}

fn tokenise_expr(s: &str) -> impl Iterator<Item = Token> + '_ {
    s.bytes().filter_map(move |b| match b {
        b' ' => None,
        b'0'..=b'9' => Some(Token::Value((b - b'0') as u64)),
        b'+' => Some(Token::Add),
        b'*' => Some(Token::Mul),
        b'(' => Some(Token::LParen),
        b')' => Some(Token::RParen),
        _ => panic!("unrecognised byte: {}", b),
    })
}

fn evaluate_tokens<I: Iterator<Item = Token>>(tokens: &mut I) -> u64 {
    let mut acc: u64 = 0;
    let mut next_op = Token::Add;
    while let Some(t) = tokens.next() {
        match t {
            Token::Value(v) => match next_op {
                Token::Add => {
                    acc += v;
                }
                Token::Mul => {
                    acc *= v;
                }
                _ => panic!("unexpected next_op: {:?}", next_op),
            },
            Token::Add | Token::Mul => {
                next_op = t;
            }
            Token::LParen => {
                let v = evaluate_tokens(tokens);
                match next_op {
                    Token::Add => {
                        acc += v;
                    }
                    Token::Mul => {
                        acc *= v;
                    }
                    _ => panic!("unexpected next_op: {:?}", next_op),
                }
            }
            Token::RParen => {
                return acc;
            }
        }
    }
    acc
}

fn evaluate_expr(s: &str) -> u64 {
    let mut tokens = tokenise_expr(s);
    evaluate_tokens(&mut tokens)
}

fn part1(input_path: PathBuf) -> crate::Result<String> {
    Ok(util::read_lines(&input_path)
        .map(|line| evaluate_expr(&line))
        .sum::<u64>()
        .to_string())
}

fn part2(input_path: PathBuf) -> crate::Result<String> {
    Err("unimplemented".into())
}

pub fn register(runner: &mut crate::Runner) {
    runner.add("day18part1", || part1(data_path!("day18_input.txt")));
    runner.add("day18part2", || part2(data_path!("day18_input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenise_expr() {
        assert_eq!(
            tokenise_expr("1 + 2 * 3 + 4 * 5 + 6").collect::<Vec<Token>>(),
            vec![
                Token::Value(1),
                Token::Add,
                Token::Value(2),
                Token::Mul,
                Token::Value(3),
                Token::Add,
                Token::Value(4),
                Token::Mul,
                Token::Value(5),
                Token::Add,
                Token::Value(6),
            ]
        );
        assert_eq!(
            tokenise_expr("1 + (2 * 3) + (4 * (5 + 6))").collect::<Vec<Token>>(),
            vec![
                Token::Value(1),
                Token::Add,
                Token::LParen,
                Token::Value(2),
                Token::Mul,
                Token::Value(3),
                Token::RParen,
                Token::Add,
                Token::LParen,
                Token::Value(4),
                Token::Mul,
                Token::LParen,
                Token::Value(5),
                Token::Add,
                Token::Value(6),
                Token::RParen,
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_evaluate_expr() {
        assert_eq!(evaluate_expr("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(evaluate_expr("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(evaluate_expr("2 * 3 + (4 * 5)"), 26);
        assert_eq!(evaluate_expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            evaluate_expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            evaluate_expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(
            part1(data_path!("day18_input.txt")).unwrap(),
            "75592527415659"
        );
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(part2(data_path!("day18_input.txt")).unwrap(), "");
    }
}
