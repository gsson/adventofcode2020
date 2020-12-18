#![feature(str_split_once)]

use std::io::Read;
use std::fmt::Debug;
use std::collections::VecDeque;

fn main() {
    let mut f = std::fs::File::open("src/bin/aoc18.txt").unwrap();
    let mut input = Vec::new();
    f.read_to_end(&mut input).unwrap();

    let o: i64 = input.split(|b| *b == b'\n')
        .filter(|i| !i.is_empty())
        .map(|i| eval(i))
        .sum();

    eprintln!("{}", o);
    assert_eq!(6811433855019, o);

}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Token {
    Literal(i64),
    Plus,
    Mul,
    LParen,
    RParen,
}

impl Token {
    fn precedence(self) -> usize {
        match self {
            Token::Literal(_) => 0,
            Token::Plus => 1,
            Token::Mul => 1,
            Token::LParen => 0,
            Token::RParen => 0,
        }
    }
}

fn tokens(mut s: &[u8]) -> impl Iterator<Item=Token> + '_ {
    fn consume_token(mut s: &[u8]) -> Option<(Token, &[u8])> {
        loop {
            match s {
                [b'(', tail @ ..] => break Some((Token::LParen, tail)),
                [b')', tail @ ..] => break Some((Token::RParen, tail)),
                [b'+', tail @ ..] => break Some((Token::Plus, tail)),
                [b'*', tail @ ..] => break Some((Token::Mul, tail)),
                [b, tail @ ..] if b.is_ascii_digit() => {
                    let (n, tail) = consume_literal(*b, tail);
                    break Some((Token::Literal(n), tail));
                }
                [b' ', tail @ ..] => s = tail,
                _ => break None
            }
        }
    }

    fn consume_literal(head: u8, mut s: &[u8]) -> (i64, &[u8]) {
        let mut n = (head - b'0') as i64;
        loop {
            match s {
                [b, tail @ ..] if b.is_ascii_digit() => {
                    n = n * 10 + (b - b'0') as i64;
                    s = tail
                }
                _ => break (n, s)
            }
        }
    }

    std::iter::from_fn(move || {
        if let Some((token, tail)) = consume_token(s) {
            s = tail;
            Some(token)
        } else {
            None
        }
    })
}



fn as_postfix(input: impl Iterator<Item=Token>) -> impl Iterator<Item=Token> {
    let mut stack: VecDeque<Token> = VecDeque::new();
    let mut input = input.peekable();
    std::iter::from_fn(move || {
        while let Some(token) = input.peek() {
            match token {
                Token::Literal(_) => return Some(input.next().unwrap()),
                Token::Plus | Token::Mul => {
                    if let Some(front) = stack.front() {
                        if front.precedence() >= token.precedence() {
                            return stack.pop_front();
                        }
                    }
                    stack.push_front(input.next().unwrap())
                }
                Token::LParen => stack.push_front(input.next().unwrap()),
                Token::RParen => {
                    if let Some(t) = stack.pop_front() {
                        if t == Token::LParen {
                            input.next().unwrap();
                        } else {
                            return Some(t);
                        }
                    }
                }
            }
        }
        stack.pop_front()
    })
}

fn eval(s: &[u8]) -> i64 {
    fn apply(stack: &mut VecDeque<i64>, token: Token) {
        match token {
            Token::Literal(n) => stack.push_front(n),
            Token::Plus => {
                let lhs = stack.pop_front().unwrap();
                let rhs = stack.pop_front().unwrap();
                stack.push_front(lhs + rhs)
            }
            Token::Mul => {
                let lhs = stack.pop_front().unwrap();
                let rhs = stack.pop_front().unwrap();
                stack.push_front(lhs * rhs)
            }
            _ => unreachable!()
        }
    }
    as_postfix(tokens(s))
        .fold(VecDeque::new(), |mut stack, token| {
            apply(&mut stack, token);
            stack
        }).pop_front().unwrap()
}


#[cfg(test)]
mod tests {

    use crate::*;

    const EXAMPLES: [(&[u8], i64); 6] = [
        (b"1 + 2 * 3 + 4 * 5 + 6", 71),
        (b"1 + (2 * 3) + (4 * (5 + 6))", 51),
        (b"2 * 3 + (4 * 5)", 26),
        (b"5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
        (b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
        (b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)
    ];

    #[test]
    fn test_tokens() {
        assert_eq!(vec![Token::Literal(1)], tokens(b"1").collect::<Vec<_>>());
        assert_eq!(vec![Token::Literal(1), Token::Plus, Token::Literal(20)], tokens(b"1 + 20").collect::<Vec<_>>());
        assert_eq!(vec![Token::Literal(1), Token::Mul, Token::Literal(20)], tokens(b"1 * 20").collect::<Vec<_>>());
        assert_eq!(vec![Token::LParen, Token::Literal(20), Token::RParen], tokens(b"(20)").collect::<Vec<_>>());
    }

    #[test]
    fn test_to_postfix() {
        assert_eq!(vec![Token::Literal(1)], as_postfix([Token::Literal(1)].iter().copied()).collect::<Vec<_>>());
        assert_eq!(vec![Token::Literal(1), Token::Literal(20), Token::Plus], as_postfix([Token::Literal(1), Token::Plus, Token::Literal(20)].iter().copied()).collect::<Vec<_>>());
        assert_eq!(vec![Token::Literal(1), Token::Literal(20), Token::Plus, Token::Literal(30), Token::Mul],
                   as_postfix([Token::Literal(1), Token::Plus, Token::Literal(20), Token::Mul, Token::Literal(30)].iter().copied()).collect::<Vec<_>>());
        assert_eq!(vec![Token::Literal(1), Token::Literal(20), Token::Literal(30), Token::Mul, Token::Plus],
                   as_postfix([Token::Literal(1), Token::Plus, Token::LParen, Token::Literal(20), Token::Mul, Token::Literal(30), Token::RParen].iter().copied()).collect::<Vec<_>>());
    }

    #[test]
    fn test_eval() {
        assert_eq!(1i64, eval(b"1"));
        assert_eq!(2i64, eval(b"1+1"));
        assert_eq!(4i64, eval(b"1+1*2"));
        assert_eq!(3i64, eval(b"1+(1*2)"));
    }

    #[test]
    fn test_examples() {
        for (expr, expected) in &EXAMPLES {
            assert_eq!(*expected, eval(*expr))
        }
    }
}
