use super::lexer::{Lexer, Operators, Token};
use super::ast::AST;

pub struct Parser<'a> {
    current_token: Token,
    lexer: Lexer<'a>,
    brackets_open: usize,
}

impl<'a> Parser<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self, String> {
        let mut lexer = Lexer::new(bytes);
        let current_token = lexer.get_next_token()?;
        Ok(Self {
            lexer,
            current_token,
            brackets_open: 0,
        })
    }

    fn get_next_token(&mut self) -> Result<Token, String> {
        self.lexer.get_next_token()
    }

    fn integer(&mut self) -> Result<AST<Token>, String> {
        match self.current_token {
            Token::Integer(num) => {
                self.current_token = self.get_next_token()?;
                return Ok(AST::new(
                    Token::Integer(num)  
                ))
            }
            Token::Operator(ref o) => match o {
                Operators::MINUS => {
                    self.current_token = self.get_next_token()?;
                    if let Token::Integer(num) = self.current_token {
                        self.current_token = self.get_next_token()?;
                        return Ok(AST::new(Token::Integer(-num)))
                    }
                }
                Operators::PLUS => {
                    self.current_token = self.get_next_token()?;
                    if let Token::Integer(num) = self.current_token {
                        self.current_token = self.get_next_token()?;
                        return Ok(AST::new(Token::Integer(num)))
                    }
                }
                _ => {}
            },
            Token::LPAREN => {
                self.eat(Token::LPAREN)?;
                self.brackets_open += 1;
                let result = self.expr()?;
                self.eat(Token::RPAREN)?;
                self.brackets_open -= 1;
                return Ok(result);
            }
            _ => {}
        }
        return Err(format!("Expected INTEGER found {:?}", self.current_token));
    }


    fn eat(&mut self, token: Token) -> Result<(), String> {
        if self.current_token == token {
            self.current_token = self.get_next_token()?;
            return Ok(());
        } else {
            return Err(format!(
                "Expected {:?} found {:?}",
                token, self.current_token
            ));
        }
    }

    fn term(&mut self) -> Result<AST<Token>, String> {
        let mut result = self.integer()?;


        while let Token::Operator(ref op) = self.current_token {
            match *op {
                Operators::MULTIPLICATION => {
                    self.current_token = self.get_next_token()?;
                    result = AST::full_self(result, Token::Operator(Operators::MULTIPLICATION), self.term()?)
                }
                Operators::DIVISION => {
                    self.current_token = self.get_next_token()?;
                    result = AST::full_self(result, Token::Operator(Operators::DIVISION), self.integer()?)
                }
                _ => { break; }
            }
        }
        Ok(result)
    }

    pub fn expr(&mut self) -> Result<AST<Token>, String> {
        let mut result = self.term()?;

        while let Token::Operator(ref op) = self.current_token {
            use Operators::*;
            match *op {
                PLUS => {
                    self.current_token = self.get_next_token()?;
                    result = AST::full_self(result, Token::Operator(PLUS), self.term()?)
                }
                MINUS => {
                    self.current_token = self.get_next_token()?;
                    result = AST::full_self(result, Token::Operator(MINUS), self.term()?)
                }
                
                _ => { break; },
            }
        }
        if let Token::RPAREN = self.current_token {
            if self.brackets_open == 0 {
                return Err(format!("Unexpected token `)` found"));
            }
        }

        Ok(result)
    }

    pub fn parse(&mut self) -> Result<AST<Token>, String> {
        let expr = self.expr()?;
        if self.current_token != Token::EOF {
            return Err(
                format!("Syntax Error, found {:?}. Expected operator", self.current_token)
            )
        }
        return Ok(expr)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Token::EOF = self.current_token {
            return None;
        } else {
            let mut return_token = self.get_next_token().unwrap();
            std::mem::swap(&mut self.current_token, &mut return_token);
            Some(return_token)
        }
    }
}

#[test]
fn generate() {
    let expr = "1+2*3-4/5".as_bytes();
    let interp = Parser::new(expr).unwrap();
    let tokens = interp.into_iter().collect::<Vec<_>>();
    assert_eq!(tokens.len(), 9);
    assert_eq!(
        vec![
            Token::Integer(1 as f64),
            Token::Operator(Operators::PLUS),
            Token::Integer(2 as f64),
            Token::Operator(Operators::MULTIPLICATION),
            Token::Integer(3 as f64),
            Token::Operator(Operators::MINUS),
            Token::Integer(4 as f64),
            Token::Operator(Operators::DIVISION),
            Token::Integer(5 as f64),
        ],
        tokens
    )
}
