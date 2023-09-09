use std::iter::Peekable;
use std::slice::Iter;
use super::ast::proc::{Procedure, ProcedureCall};

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    line_no: usize,
    column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexer: &Lexer, len: Option<usize>) -> Self{
        let offset = if len.is_some(){ 
            unsafe{len.unwrap_unchecked()}
        } else {
            0
        };
        Self {
            token_type,
            line_no: lexer.line_no,
            column: lexer.column - offset
        }
    }

    pub fn new_with_details(token_type: TokenType, line_no: usize, column: usize) -> Self{
        Self {
            token_type,
            line_no,
            column
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn token_type_mut(&mut self) -> &mut TokenType {
        &mut self.token_type
    }

    pub fn line_no(&self) -> usize{
        self.line_no
    }
    
    pub fn column(&self) -> usize {
        self.column
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    EOF,
    Integer(f64),
    Real(usize),
    Operator(Operators),
    LPAREN,
    COLON,
    RPAREN,
    BEGIN,
    END,
    IDENTIFIER(String),
    SEMICOLON,
    DOT,
    ASSIGN,
    EMPTY,
    PROGRAM,
    COMMA,
    VAR,
    PROCEDURE(Option<Box<Procedure>>),
    PROCEDURECALL(Box<ProcedureCall>),
}

impl TokenType{
    pub fn equal(&self, other: &Self) -> bool {
        match (self,other) {
            (Self::EOF, Self::EOF) => true,
            (Self::Integer(_),Self::Integer(_)) => true,      
            (Self::Real(_),Self::Real(_)) => true,
            (Self::Operator(_),Self::Operator(_)) => true,    
            (Self::LPAREN, Self::LPAREN) => true,
            (Self::COLON, Self::COLON) => true,
            (Self::RPAREN, Self::RPAREN) => true,
            (Self::BEGIN, Self::BEGIN) => true,
            (Self::END, Self::END) => true,
            (Self::IDENTIFIER(_),Self::IDENTIFIER(_)) => true,
            (Self::SEMICOLON, Self::SEMICOLON) => true,       
            (Self::DOT, Self::DOT) => true,
            (Self::ASSIGN, Self::ASSIGN) => true,
            (Self::EMPTY, Self::EMPTY) => true,
            (Self::PROGRAM, Self::PROGRAM) => true,
            (Self::COMMA, Self::COMMA) => true,
            (Self::VAR, Self::VAR) => true,
            (Self::PROCEDURE(_),Self::PROCEDURE(_)) => true,
            _ => false
        }
    }
    
    pub fn is_procedure(&self) -> bool{
        match self {
            Self::PROCEDURE(_) => true,
            _ => false
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operators {
    PLUS,
    MINUS,
    IDIVISION,
    FDIVISION,
    MULTIPLICATION,
}

pub struct Lexer<'a> {
    stream: Peekable<Iter<'a, u8>>,
    current_char: Option<char>,
    next_char: Option<char>,
    line_no: usize,
    column: usize
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
            next_char,
            line_no: 1,
            column: 1
        }
    }

    fn advance(&mut self) {
        self.column += 1;
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
        let len = result.len();
        match &*result {
            "BEGIN" => Token::new(TokenType::BEGIN, &self, Some(result.len())),
            "END" => Token::new(TokenType::END, &self, Some(result.len())),
            "DIV" => Token::new(TokenType::Operator(Operators::IDIVISION), &self, Some(result.len())),
            "PROGRAM" => Token::new(TokenType::PROGRAM, &self, Some(result.len())),
            "PROCEDURE" => Token::new(TokenType::PROCEDURE(
                None
            ),&self, Some(result.len())),
            "VAR" => Token::new(TokenType::VAR, &self, Some(result.len())),
            "REAL" => Token::new(TokenType::Real(0), &self, Some(result.len())),
            "INTEGER" => Token::new(TokenType::Integer(0.0), &self, Some(result.len())),
            _ => Token::new(TokenType::IDENTIFIER(result), &self, Some(len))
        }

    }

    fn skip_whitespace(&mut self) {
        while let Some(char) = self.current_char {
            if char == '\n'{
                self.column = 0;
                self.line_no += 1;
                self.advance()
            }
            else if char.is_ascii_whitespace() {
                self.advance()
            } else {
                break;
            }
        }
    }

    fn integer(&mut self) -> Result<Token, String>{
        let mut int = String::new();
        int.push(self.current_char.unwrap());
        self.advance();
        let mut dot_count = 0;
        while let Some(val) = self.current_char {
            if val.is_numeric() {
                int.push(val);
                self.advance()
            } else if val == '.'{
                if dot_count == 0 {
                    int.push(val);
                    dot_count += 1;
                    self.advance();
                } else {
                    return Err(format!("Cannot have two dots in integer. Position line_no: {}, column: {}",self.line_no, self.column))
                }
            } else {
                break;
            }
        }
        return Ok(
            Token::new(
                TokenType::Integer(int.parse::<f64>().unwrap()), &self, Some(int.len())
            )
        );
    }

    pub fn skip_comment(&mut self) {
        while self.current_char != Some('}') {
            self.advance()
        }
        self.advance();
    }

    pub fn get_current_character(&mut self) -> char {
        self.skip_whitespace();
        self.current_char.as_ref().unwrap_or(&' ').clone()
    }

    pub fn get_next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        match self.current_char {
            Some('+') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::Operator(Operators::PLUS), &self, Some(1)
                    )
                )
            }
            Some('-') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::Operator(Operators::MINUS), &self, Some(1)
                    )
                )
            }
            Some('*') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::Operator(Operators::MULTIPLICATION), &self, Some(1)
                    )
                )
            }
            Some('/') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::Operator(Operators::FDIVISION), &self, Some(1)
                    )
                )
            }
            Some('(') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::LPAREN, &self, Some(1)
                    )
                )
            }
            Some(')') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::RPAREN, &self, Some(1)
                    )
                )
            }
            Some('.') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::DOT, &self, Some(1)
                    )
                )
            }
            Some(',') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::COMMA, &self, Some(1)
                    )
                )
            }
            Some(';') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::SEMICOLON, &self, Some(1)
                    )
                )
            }
            Some(':') if self.next_char == Some('=') => {
                self.advance();
                self.advance();
                Ok(
                    Token::new(
                        TokenType::ASSIGN, &self, Some(2)
                    )
                )
            }
            Some(':') => {
                self.advance();
                Ok(
                    Token::new(
                        TokenType::COLON, &self, Some(1)
                    )
                )
            }
            Some('{') => {
                self.advance();
                self.skip_comment();
                return self.get_next_token();
            }

            Some(char) => {
                if char.is_numeric() {
                    Ok(self.integer()?)
                } else if char.is_alphabetic() || char == '_' {
                    Ok(self.id())
                } 
                else {
                    Err(format!("Cannot parse {}. Position: line_no: {}, column: {}", char, self.line_no, self.column))
                }
            }
            None => Ok(
                Token::new(
                    TokenType::EOF, &self, Some(1)
                )
            )
        }
    }
}
