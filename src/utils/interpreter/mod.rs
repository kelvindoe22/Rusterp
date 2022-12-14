use super::lexer::{Lexer, Operators, Token};

pub struct Interpreter<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    brackets_open: usize
}

impl<'a> Interpreter<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self, String> {
        let mut lexer = Lexer::new(bytes);
        let current_token = lexer.get_next_token()?;
        Ok(Self {
            lexer,
            current_token,
            brackets_open: 0
        })
    } 

    fn get_next_token(&mut self) -> Result<Token, String> {
        self.lexer.get_next_token()
    }

    fn integer(&mut self) -> Result<f64, String> {
        match self.current_token {
            Token::Integer(num) => {
                let res = Ok(num);
                self.current_token = self.get_next_token()?;
                return res
            }
            Token::Operator(ref o) => {
                match o {
                    Operators::MINUS => {
                        self.current_token = self.get_next_token()?;
                        if let Token::Integer(num) = self.current_token {
                            self.current_token = self.get_next_token()?;
                            return Ok(-num)
                        }
                    }
                    Operators::PLUS => {
                        self.current_token = self.get_next_token()?;
                        if let Token::Integer(num) = self.current_token {
                            self.current_token = self.get_next_token()?;
                            return Ok(num)
                        }
                    }
                    _ => {}
                }
            }
            Token::LPAREN => {
                self.eat(Token::LPAREN)?;
                dbg!("brackets open");
                self.brackets_open += 1;
                let result = self.expr()?;
                self.eat(Token::RPAREN)?;
                self.brackets_open -= 1;
                dbg!("closed");
                return Ok(result);
            }
            _ => {
            }
        }
        return Err(format!("Expected INTEGER found {:?}", self.current_token))
    }

    fn operator(&mut self, operator: Operators) -> Result<(), String> {
        if let Token::Operator(ref o) = self.current_token {
            if o == &operator {
                self.current_token = self.get_next_token()?;
                Ok(())
            } else {
                Err(format!("Expected Operator {:?} found {:?}", operator, o))
            }
        } else {
            Err(format!(
                "Expected {:?} found {:?}",
                Token::Operator(operator),
                self.current_token
            ))
        }
    }

    fn eat(&mut self, token: Token) -> Result<(), String> {
        if self.current_token == token{
            self.current_token = self.get_next_token()?;
            return Ok(())
        }else {
            return Err(
                format!(
                    "Expected {:?} found {:?}",token,self.current_token
                )
            )
        }
    }

    fn term(&mut self) -> Result<f64, String> {
        let mut result = self.integer()?;
        
        while let Token::Operator(ref op) = self.current_token {
            match *op {
                Operators::MULTIPLICATION => {
                    self.operator(Operators::MULTIPLICATION)?;
                    result *= self.integer()?;
                }
                Operators::DIVISION => {
                    self.operator(Operators::DIVISION)?;
                    result /= self.integer()?;
                }
                _ => break,
            }
        }
        Ok(result)
    }

    pub fn expr(&mut self) -> Result<f64, String> {
        let mut result = self.term()?;

        while let Token::Operator(ref op) = self.current_token {
            use Operators::*;
            match *op {
                PLUS => {
                    self.operator(PLUS)?;
                    result += self.term()?;
                }
                MINUS => {
                    self.operator(MINUS)?;
                    result -= self.term()?;
                }
                _ => break,
            }
        }
        if let Token::RPAREN = self.current_token {
            if self.brackets_open == 0{
                return Err(format!("Unexpected token `)` found"))
            }
        }
        Ok(result)
    }
}

impl<'a> Iterator for Interpreter<'a> {
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
    let interp = Interpreter::new(expr).unwrap();
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
