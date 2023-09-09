use std::collections::HashMap;

use crate::utils::ast::proc::ProcedureCall;

use super::ast::block::Block;
use super::ast::proc::Procedure;
use super::lexer::{Lexer, Operators, Token, TokenType};
use super::ast::ast::AST;
use super::ast::program::Program;
use super::err::functions::better_error;

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
    
    fn eat(&mut self, token: TokenType) -> Result<(), String> {
        if self.current_token.token_type().equal(&token) {
            self.current_token = self.get_next_token()?;
            return Ok(());
        } else {
            return Err(format!(
                "Expected {:?} found {:?}. Position: line_no: {}, column: {}",
                token, self.current_token, self.current_token.line_no(), self.current_token.column()
            ));
        }
    }

    fn get_next_token(&mut self) -> Result<Token, String> {
        self.lexer.get_next_token()
    }

    fn integer(&mut self) -> Result<AST<Token>, String> {
        match self.current_token.token_type() {
            TokenType::Integer(_) => {
                let mut old_token = self.get_next_token()?;
                std::mem::swap(&mut old_token, &mut self.current_token);
                return Ok(AST::new(
                    old_token
                ))
            }
            TokenType::IDENTIFIER(_) => {
                let mut old_token = self.get_next_token()?;
                std::mem::swap(&mut old_token, &mut self.current_token);
                return Ok(
                    AST::new(old_token)
                );
            }
            TokenType::Operator(ref o) => match o {
                Operators::MINUS => {
                    let mut old_token = self.get_next_token()?;
                    std::mem::swap(&mut old_token, &mut self.current_token);

                    return Ok(
                        AST::unary(
                            old_token,self.term()?
                        )
                    )
 
                }
                Operators::PLUS => {
                    let mut old_token = self.get_next_token()?;
                    std::mem::swap(&mut old_token, &mut self.current_token);

                    return Ok(
                        AST::unary(old_token, self.term()?)
                    )
                }
                _ => {}
            },
            TokenType::LPAREN => {
                self.eat(TokenType::LPAREN)?;
                self.brackets_open += 1;
                let result = self.expr()?;
                self.eat(TokenType::RPAREN)?;
                self.brackets_open -= 1;
                return Ok(result);
            }
            _ => {}
        }
        return Err(
            better_error(
                format!("Expected INTEGER found {:?}", self.current_token.token_type()),
                &self.current_token
            )
        );
    }



    fn term(&mut self) -> Result<AST<Token>, String> {
        let mut result = self.integer()?;


        while let TokenType::Operator(ref op) = self.current_token.token_type() {
            match *op {
                Operators::MULTIPLICATION => {
                    let mut old_token = self.get_next_token()?;
                    std::mem::swap(&mut old_token, &mut self.current_token);
                    result = AST::full_self(result, old_token, self.term()?)
                }
                Operators::FDIVISION => {
                    let mut old_token = self.get_next_token()?;
                    std::mem::swap(&mut old_token, &mut self.current_token);
                    self.current_token = self.get_next_token()?;
                    result = AST::full_self(result, old_token, self.integer()?)
                }
                Operators::IDIVISION => {
                    let mut old_token = self.get_next_token()?;
                    std::mem::swap(&mut old_token, &mut self.current_token);
                    result = AST::full_self(result, old_token, self.integer()?)
                }
                _ => { break; }
            }
        }
        Ok(result)
    }

    pub fn program(&mut self) -> Result<Program, String> {
        self.eat(TokenType::PROGRAM)?;
        let name = match &self.current_token.token_type() {
            TokenType::IDENTIFIER(s) => {s.clone()},
            e => return Err(
                better_error(
                format!("Did not find name of the program found {:?}", e),
                &self.current_token
                )
            )
        };
        self.current_token = self.get_next_token()?;
        self.eat(TokenType::SEMICOLON)?;
        Ok(Program::new(
            name, 
            self.block()?
        ))
    }

    fn declarations(&mut self) -> Result<HashMap<String, Token>, String> {
        self.eat(TokenType::VAR)?;
        let mut map = HashMap::new();
        while let TokenType::IDENTIFIER(_) = self.current_token.token_type() {
            map.extend(self.vardeclarations(true)?)
        }
        Ok(map)
    }

    fn vardeclarations(&mut self, semi_required: bool) -> Result<HashMap<String, Token>, String> {
        let mut idents = Vec::new();
        match &self.current_token.token_type() {
            TokenType::IDENTIFIER( i) => {idents.push(i.clone())}
            e => return Err(
                better_error(
                    format!("Expected Identifier found {:?}", e),
                    &self.current_token
                )
            )
        }
        self.current_token = self.get_next_token()?;
        while let TokenType::COMMA = self.current_token.token_type() {
            self.eat(TokenType::COMMA)?;
            match &self.current_token.token_type() {
                TokenType::IDENTIFIER(ref i) => {idents.push(i.clone())}
                e => return Err(better_error(
                    format!("Expected Identifier found {:?}", e),
                    &self.current_token
                ))
            }
            self.current_token = self.get_next_token()?;

        }
        
        if let TokenType::COLON = self.current_token.token_type() { 
            self.current_token = self.get_next_token()?;   
        } else {
           return Err(better_error(
            format!("Expected `:` found {:?}", self.current_token),
            &self.current_token
        ))
        }

        
        let mut data_type = self.get_next_token()?;
        std::mem::swap(&mut self.current_token, &mut data_type);
        if !semi_required {
            if let TokenType::SEMICOLON = self.current_token.token_type()  {
                self.eat(TokenType::SEMICOLON)?;
            }
        }else {
            self.eat(TokenType::SEMICOLON)?;
        }
        Ok(idents.into_iter().map(|e| {(e, data_type.clone())}).collect::<HashMap<_,_>>())
    }

    fn block(&mut self) -> Result<Block, String> {
        Ok(Block::new(
            self.declarations()?,
            self.compound()?
        ))
    }


    pub fn expr(&mut self) -> Result<AST<Token>, String> {
        let mut result = self.term()?;

        while let TokenType::Operator(ref op) = self.current_token.token_type() {
            use Operators::*;
            match *op {
                PLUS => {
                    let mut old_token = self.get_next_token()?;
                    std::mem::swap(&mut old_token, &mut self.current_token);
                    result = AST::full_self(result, old_token, self.term()?)
                }
                MINUS => {
                    let mut old_token = self.get_next_token()?;
                    std::mem::swap(&mut old_token, &mut self.current_token);
                    result = AST::full_self(result, old_token, self.term()?)
                }
                
                _ => { break; },
            }
        }
        if let TokenType::RPAREN = self.current_token.token_type() {
            if self.brackets_open == 0 {
                return Err(
                    format!(
                        "Unexpected token `)` found. Position: line_no: {}, column: {}", 
                        self.current_token.line_no(), self.current_token.column()
                    )
                );
            }
        }

        Ok(result)
    }

    fn compound(&mut self) -> Result<AST<Token>, String> {
        let new_begin = self.current_token.clone();
        self.eat(TokenType::BEGIN)?;
        let nodes = self.statement_nodes()?;
        let node = AST::new_with_children(new_begin, nodes);
        self.eat(TokenType::END)?;
        Ok(node)
    }

    fn statement_nodes(&mut self) -> Result<Vec<AST<Token>>, String> {
        let mut nodes = Vec::new();
        nodes.push(self.statement()?);
        while let TokenType::SEMICOLON = self.current_token.token_type(){
            self.eat(TokenType::SEMICOLON)?;
            nodes.push(self.statement()?);
        }
        Ok(nodes)

    }

    fn statement(&mut self) -> Result<AST<Token>, String> {
        match self.current_token.token_type() {
            TokenType::BEGIN => self.compound(),
            TokenType::IDENTIFIER(_) => {
                if self.lexer.get_current_character() == '(' {
                    return self.procedure_call();
                }
                self.assignment_statement()
            },
            TokenType::PROCEDURE(_) => self.procedure(),
            _  => Ok(
                    AST::new(
                        Token::new_with_details(
                            TokenType::EMPTY, 
                            self.current_token.line_no(), 
                            self.current_token.column()
                        ) 
                    )
                )
        } 
    }

    fn procedure_call(&mut self) -> Result<AST<Token>, String> {
        let column = self.current_token.column();
        let line_no = self.current_token.line_no();
        let procedure_name =  match self.current_token.token_type() {
            TokenType::IDENTIFIER(name) => name.clone(),
            e => return Err(
                better_error(
                    format!("Expected identifier found {:?}", e), &self.current_token 
                )
            )
        };
        self.current_token = self.get_next_token()?;
        self.eat(TokenType::LPAREN)?;
        self.brackets_open += 1;
        let parameters = if let TokenType::RPAREN = self.current_token.token_type() {
            None
        } else {
            self.procedure_parameters()?
        };
        self.eat(TokenType::RPAREN)?;
        self.brackets_open -= 1;
        Ok(
            AST::new(
                Token::new_with_details(
                    TokenType::PROCEDURECALL(
                        Box::new(
                            ProcedureCall {
                                name: procedure_name,
                                params: parameters
                            }
                        )
                    ), line_no, column
                )
            )
        )
    }

    fn procedure_parameters(&mut self) -> Result<Option<Vec<AST<Token>>>, String> {
        let mut tokens = Vec::new();
        tokens.push(self.expr()?);
        while let TokenType::COMMA = *self.current_token.token_type() {
            tokens.push(self.expr()?)
        }
        Ok(Some(tokens))
    }


    fn procedure(&mut self) -> Result<AST<Token>, String> {
        let line_no =  self.current_token.line_no();
        let column = self.current_token.column();
        self.current_token = self.get_next_token()?;
        let name = match &self.current_token.token_type() {
            TokenType::IDENTIFIER(string) => string.clone(),
            _ => return Err(better_error(
                format!("Expected identifier found {:?}", self.current_token),
                &self.current_token
            ))
        };
        let parameters = self.get_parameters()?;
        let block = self.block()?;
        Ok(AST::new(
            Token::new_with_details(
                TokenType::PROCEDURE(
                    Some(Box::new(
                        Procedure::new(name,parameters,block)?
                    ))
                ),
                line_no, 
                column)
            )
        )
    }

    fn procedure_declarations(&mut self) -> Result<Vec<(String, Token)>, String> {
        let mut idents = Vec::new();
        match &self.current_token.token_type() {
            TokenType::IDENTIFIER( i) => {idents.push(i.clone())}
            e => return Err(format!("Expected Identifier found {:?}", e))
        }
        self.current_token = self.get_next_token()?;
        while let TokenType::COMMA = self.current_token.token_type() {
            self.eat(TokenType::COMMA)?;
            match &self.current_token.token_type() {
                TokenType::IDENTIFIER(ref i) => {idents.push(i.clone())}
                e => return Err(format!("Expected Identifier found {:?}", e))
            }
            self.current_token = self.get_next_token()?;

        }
        
        if let TokenType::COLON = self.current_token.token_type() { 
            self.current_token = self.get_next_token()?;   
        } else {
           return Err(
            better_error(
                format!("Expected `:` found {:?}", self.current_token),&self.current_token
                )
            )
        }

        
        let mut data_type = self.get_next_token()?;
        std::mem::swap(&mut self.current_token, &mut data_type);

        if let TokenType::SEMICOLON = self.current_token.token_type()  {
            self.eat(TokenType::SEMICOLON)?;
        }
        
        Ok(idents.into_iter().map(|e| {(e, data_type.clone())}).collect::<Vec<_>>())
    
    }

    fn get_parameters(&mut self) -> Result<Option<Vec<(String, Token)>>, String> {
        self.current_token = self.get_next_token()?;
        match &self.current_token.token_type() {
            TokenType::SEMICOLON => return Ok(None),
            TokenType::LPAREN => {
                let mut vec = Vec::new();
                self.current_token = self.get_next_token()?;
                while let TokenType::IDENTIFIER(_) = self.current_token.token_type() {
                    vec.extend(
                        self.procedure_declarations()?.into_iter()
                    );
                }
                self.eat(TokenType::RPAREN)?;
                self.eat(TokenType::SEMICOLON)?;
                Ok(Some(vec))
            }
            
            e => return Err(better_error(
                format!("Expected `,` or `(` found {:?}",e),
                &self.current_token
            ))
        }
    }

    fn identifier(&mut self) -> Result<AST<Token>, String>{
        let mut next_token = self.get_next_token()?;
        std::mem::swap(&mut self.current_token, &mut next_token);
        match next_token.token_type() {
            TokenType::IDENTIFIER(_) => {
                Ok(AST::new(next_token.clone()))
            }
            token => {
                Err(better_error(
                    format!("Expected identifier found {:?}", token),
                    &self.current_token
                ))
            }
        }
    }

    fn assignment_statement(&mut self) -> Result<AST<Token>, String> {
        let left = self.identifier()?;
        let center = self.current_token.clone();
        self.eat(TokenType::ASSIGN)?;
        let right = self.expr()?;
        let mut children = Vec::with_capacity(2);
        children.push(left);
        children.push(right);
        Ok(AST::new_with_children(center, children))
    }



}

impl<'a> Iterator for Parser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let TokenType::EOF = self.current_token.token_type() {
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
    let expr = "1 + 2 * 3 - 4 DIV 5 ".as_bytes();
    let interp = Parser::new(expr).unwrap();
    let tokens = interp.into_iter().collect::<Vec<_>>();
    assert_eq!(tokens.len(), 9);
}
