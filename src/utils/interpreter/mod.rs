use super::lexer::{Lexer, Token, Operators};


pub struct Interpreter<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Interpreter<'a> {
    pub fn new(bytes: &'a [u8]) -> Result<Self, String> {
        let mut lexer = Lexer::new(bytes);
        let current_token = lexer.get_next_token()?;
        Ok(Self {
            lexer,
            current_token
        })
    }

    fn get_next_token(&mut self) -> Result<Token, String> {
        self.lexer.get_next_token()
    }
    
    fn integer(&mut self) -> Result<f64, String> {
        if let Token::Integer(num) = self.current_token {
            let res = Ok(num);
            self.current_token = match self.get_next_token() {
                Ok(token) => token,
                Err(_) => return Err(String::from("Can't process next token"))
            };
            res
        } else{
            Err(format!("Expected INTEGER found {:?}",self.current_token))
        }
        
    }
    
    fn operator(&mut self, operator: Operators) -> Result<(), String> {
        if let Token::Operator( ref o) = self.current_token {
            if o == &operator {
                self.current_token = match self.get_next_token() {
                    Ok(token) => token,
                    Err(_) => return Err(String::from("Can't process next token"))
                };
                Ok(())
            }else {
                Err(format!("Expected Operator {:?} found {:?}",operator, o))
            }
        } else{
            Err(format!("Expected {:?} found {:?}",Token::Operator(operator),self.current_token))
        }
        
    }

    fn term(&mut self) -> Result<f64,String> {
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
        while let Token::Operator(ref op)  = self.current_token {
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
        Ok(result)
    }

}

impl<'a> Iterator for Interpreter<'a>{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Token::EOF = self.current_token{
            return None;
        } else {
            let mut return_token = self.get_next_token().unwrap();
            std::mem::swap(&mut self.current_token, &mut return_token);
            Some(return_token)
        }
    }
}


#[test]
fn generate(){
    let expr = "1+2*3-4/5".as_bytes();
    let interp = Interpreter::new(expr).unwrap();
    let tokens = interp.into_iter().collect::<Vec<_>>();
    assert_eq!(tokens.len(), 9);
    assert_eq!(
        vec![
            Token::Integer(1 as f64), Token::Operator(Operators::PLUS),
            Token::Integer(2 as f64), Token::Operator(Operators::MULTIPLICATION),
            Token::Integer(3 as f64), Token::Operator(Operators::MINUS), 
            Token::Integer(4 as f64), Token::Operator(Operators::DIVISION),
            Token::Integer(5 as f64), 
        ],
        tokens
    )
}