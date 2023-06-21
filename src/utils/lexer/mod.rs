use std::slice::Iter;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Integer(f64),
    Operator(Operators),
    LPAREN,
    RPAREN,
}

impl Add for Token {
    type Output = f64;
    fn add(self, rhs: Self) -> Self::Output {
        use Token::*;
        match self{
            Integer(left) => {
                match rhs {
                    Integer(right) => left + right,
                    _ => panic!("Cannot add {:?} to {:?}.", left, rhs)
                }
            }
            _ => {
                panic!("Cannot add {:?} to {:?}", self, rhs);
            }

        }
    }
}

impl Sub for Token {
    type Output = f64;
    fn sub(self, rhs: Self) -> Self::Output {
        use Token::*;
        match self{
            Integer(left) => {
                match rhs {
                    Integer(right) => left - right,
                    _ => panic!("Cannot subtract {:?} to {:?}.", left, rhs)
                }
            }
            _ => {
                panic!("Cannot subtract {:?} to {:?}", self, rhs);
            }

        }
    }
}

impl Mul for Token {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        use Token::*;
        match self{
            Integer(left) => {
                match rhs {
                    Integer(right) => left * right,
                    _ => panic!("Cannot multiply {:?} by {:?}.", left, rhs)
                }
            }
            _ => {
                panic!("Cannot multiply {:?} by {:?}", self, rhs);
            }

        }
    }
}

impl Div for Token {
    type Output = f64;
    fn div(self, rhs: Self) -> Self::Output {
        use Token::*;
        match self{
            Integer(left) => {
                match rhs {
                    Integer(right) => left / right,
                    _ => panic!("Cannot divide {:?} by {:?}.", left, rhs)
                }
            }
            _ => {
                panic!("Cannot divide {:?} by {:?}", self, rhs);
            }

        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operators {
    PLUS,
    MINUS,
    DIVISION,
    MULTIPLICATION,
}

pub struct Lexer<'a> {
    stream: Iter<'a, u8>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let mut iter = bytes.iter();
        let char = match iter.next() {
            Some(val) => Some(*val as char),
            None => None,
        };
        Self {
            stream: iter,
            current_char: char,
        }
    }

    fn advance(&mut self) {
        self.current_char = match self.stream.next() {
            Some(val) => Some(*val as char),
            None => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(' ') = self.current_char {
            self.advance()
        }
    }

    fn integer(&mut self) -> Token {
        let mut int = String::new();
        int.push(self.current_char.unwrap());
        self.advance();
        while let Some(val) = self.current_char {
            if val.is_numeric() {
                int.push(val);
                self.advance()
            } else {
                break;
            }
        }
        return Token::Integer(int.parse::<f64>().unwrap());
    }

    pub fn get_next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        match self.current_char {
            Some('+') => {
                self.advance();
                Ok(Token::Operator(Operators::PLUS))
            }
            Some('-') => {
                self.advance();
                Ok(Token::Operator(Operators::MINUS))
            }
            Some('*') => {
                self.advance();
                Ok(Token::Operator(Operators::MULTIPLICATION))
            }
            Some('/') => {
                self.advance();
                Ok(Token::Operator(Operators::DIVISION))
            }
            Some('(') => {
                self.advance();
                Ok(Token::LPAREN)
            }
            Some(')') => {
                self.advance();
                Ok(Token::RPAREN)
            }

            Some(char) => {
                if char.is_numeric() {
                    Ok(self.integer())
                } else {
                    Err(format!("Cannot parse {}", char))
                }
            }
            None => Ok(Token::EOF),
        }
    }
}
