use crate::utils::lexer::{Token, TokenType};

use super::{block::Block, ast::AST, proc::Procedure};

pub struct Program {
    name: String,
    block: Block
}

impl Program {
    pub fn new(name: String, block: Block ) -> Self{
        Self {
            name,
            block
        }
    }

    pub fn scope(&self) {
        println!("{:?}",&self.block.declarations.borrow())
    }

    pub fn set_proc(&self, proc: Token) -> Result<(), String> {
        assert!(proc.token_type().is_procedure());
        self.block.set_proc(proc)
    }

    pub fn set_var(&self, ident: &Token, val: f64) -> Result<(), String>{
        self.block.set_var(ident, val)
    }

    pub fn get_num(&self, ident: &String) -> Option<f64> {
        self.block.get_num(ident)
    }

    pub fn statements(&self) -> &AST<Token> {
        return self.block.statements()
    }

}