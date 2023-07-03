use std::iter::Peekable;
use std::slice::Iter;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Integer(f64),
    Operator(Operators),
    LPAREN,
    RPAREN,
    BEGIN,
    END,
    IDENTIFIER(String),
    SEMICOLON,
    DOT,
    ASSIGN,
    EMPTY
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
    stream: Peekable<Iter<'a, u8>>,
    current_char: Option<char>,
    next_char: Option<char>
}

impl<'a> Lexer<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let mut iter = bytes.iter().peekable();
        let current_char = match iter.next() {
            Some(val) => Some(*val as char),
            None => None,
        };
        let next_char = match iter.peek(){
            Some(val) => Some(**val as char),
            None => None
        };
        Self {
            stream: iter,
            current_char,
            next_char
        }
    }

    fn advance(&mut self) {
        self.current_char = match self.stream.next() {
            Some(val) => Some(*val as char),
            None => None,
        };
        self.next_char = match self.stream.peek() {
            Some( val) => Some(**val as char),
            None => None,
        }
    }

    fn id(&mut self) -> Token {
        let mut result = String::new();
        while let Some(char) = self.current_char {
            if char.is_alphanumeric() || char == '_' {
                result.push(char);
                self.advance()
            }else {
                break
            }
        }
        match &*result {
            "BEGIN" => Token::BEGIN,
            "END" => Token::END,
            "div" => Token::Operator(Operators::DIVISION),
            _ => Token::IDENTIFIER(result)
        }

    }

    fn skip_whitespace(&mut self) {
        while let Some(char) = self.current_char {
            if char.is_ascii_whitespace() {
                self.advance()
            } else {
                break;
            }
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
            // Some('/') => {
            //     self.advance();
            //     Ok(Token::Operator(Operators::DIVISION))
            // }
            Some('(') => {
                self.advance();
                Ok(Token::LPAREN)
            }
            Some(')') => {
                self.advance();
                Ok(Token::RPAREN)
            }
            Some('.') => {
                self.advance();
                Ok(Token::DOT)
            }
            Some(';') => {
                self.advance();
                Ok(Token::SEMICOLON)
            }

            Some(':') if self.next_char == Some('=') => {
                self.advance();
                self.advance();
                Ok(Token::ASSIGN)
            }

            Some(char) => {
                if char.is_numeric() {
                    Ok(self.integer())
                } else if char.is_alphabetic() || char == '_' {
                    Ok(self.id())
                } 
                else {
                    Err(format!("Cannot parse {}", char))
                }
            }
            None => Ok(Token::EOF),
        }
    }
}
