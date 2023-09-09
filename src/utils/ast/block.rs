use std::cell::RefCell;
use std::collections::HashMap;

use crate::utils::lexer::TokenType;

use super::ast::AST;
use super::super::lexer::Token;

#[derive(Debug,Clone)]
pub struct Block {
    pub declarations: RefCell<HashMap<String, Token>>,
    statements: AST<Token>
}

impl Block {
    pub fn new(declarations: HashMap<String, Token>, statements: AST<Token>) -> Self{
        Self {
            declarations: RefCell::new(declarations),
            statements
        }
    }


    pub fn statements(&self) -> &AST<Token> {
        return &self.statements;
    }

    pub fn set_proc(&self, proc: Token) -> Result<(), String> {
        let name = match proc.token_type() {
            TokenType::PROCEDURE(p) => { 
                match p{
                    Some(procedure) => {
                        procedure.get_name()
                    }
                    None => unreachable!()
                }
            }
            _ => unreachable!()
        };
        self.declarations.borrow_mut().insert(
            name,
            proc
        );
        
        Ok(())
    }

    pub fn set_var(&self, token: &Token, val: f64) -> Result<(), String>{
        let ident = match token.token_type() {
            TokenType::IDENTIFIER(e) => e.clone(),
            _ => unreachable!()
        };
        match self.declarations.borrow_mut().get_mut(&ident) {
            None => return Err(format!("Variable {} not found.",ident)),
            Some(e) => match (e.line_no(), e.column(), e.token_type_mut()) {
                (_, _, TokenType::Real(num)) => *num = val as usize,
                (_, _, TokenType::Integer(num)) => *num = val,
                (l,c, e) => return Err(
                        format!(
                            "Expected Real or Integer found {:?}. Position line_no: {}, column: {}",e, l, c)
                        ), 
                        
            }
        }
        Ok(())
    }

    pub fn contains(&self, var:&String) -> bool{
        self.declarations.borrow().contains_key(var)
    }

    pub fn get_num(&self, ident: &String) -> Option<f64> {
        match self.declarations.borrow().get(ident){
            None => None,
            Some(token) => {
                match token.token_type() {
                    TokenType::Integer(i) => Some(i.clone()),
                    TokenType::Real(r) => Some(r.clone() as f64),
                    _ => None
                }
            }
        }
    }
    
    
}
