use std::fmt::format;

use super::lexer:: {Token, Operators};
use super::ast::AST;
use super::parser::Parser;

pub struct Interpreter{
    parser: AST<Token>
}

impl Interpreter {
    pub fn new<'a>(bytes: &'a [u8]) -> Result<Self, String> {
        Ok(Self {
            parser : Parser::new(bytes)?.parse()?
        })
    }

    pub fn interprete(&self) -> Result<f64, String> {
        Self::visit_node(&self.parser)
    }

    fn visit_node(node: &AST<Token>) -> Result<f64, String>{
        use Operators::*;
        match *node.view() {
            Token::Integer(num) => return Ok(num),
            Token::Operator(ref op) => {
                match *op{
                    PLUS => return Ok(Self::visit_node(node.left().unwrap().as_ref())? + Self::visit_node(node.right().unwrap().as_ref())?),
            
                    MINUS => return Ok(Self::visit_node(node.left().unwrap().as_ref())? - Self::visit_node(node.right().unwrap().as_ref())?),
                    
                    MULTIPLICATION => return Ok(Self::visit_node(node.left().unwrap().as_ref())? * Self::visit_node(node.right().unwrap().as_ref())?),
                    
                    DIVISION => return Ok(Self::visit_node(node.left().unwrap().as_ref())? / Self::visit_node(node.right().unwrap().as_ref())?),

                }
            }
            _ => { unreachable!() }
        }
    }

    fn rewrite(node: &AST<Token>) -> Result<String, String> {
        use Operators::*;
        match *node.view() {
            Token::Integer(num) => return Ok(format!("{}",num)),
            Token::Operator(ref op) => {
                match *op{
                    PLUS => return Ok(format!("({} + {})",Self::rewrite(node.left().unwrap().as_ref())?,Self::rewrite(node.right().unwrap().as_ref())?)),
                    MINUS => return Ok(format!("({} - {})",Self::rewrite(node.left().unwrap().as_ref())?,Self::rewrite(node.right().unwrap().as_ref())?)),
                    MULTIPLICATION => return Ok(format!("({} * {})",Self::rewrite(node.left().unwrap().as_ref())?,Self::rewrite(node.right().unwrap().as_ref())?)),
                    DIVISION => return Ok(format!("({} / {})",Self::rewrite(node.left().unwrap().as_ref())?,Self::rewrite(node.right().unwrap().as_ref())?)),
                }
            }
            _ => { unreachable!() }
        }
    }

    pub fn spit(&self) -> Result<String, String>{
        Self::rewrite(&self.parser)
    }

}

