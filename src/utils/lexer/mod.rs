use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Integer(f64),
    Operator(Operators),
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
