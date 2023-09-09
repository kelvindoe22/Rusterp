use crate::utils::{lexer::Token, err::functions::better_error};
use super::{block::Block, ast::AST};

#[derive(Debug, Clone)]
pub struct ProcedureCall {
    pub name: String,
    pub params: Option<Vec<AST<Token>>>
}

#[derive(Debug, Clone)]
pub struct Procedure {
    name: String,
    params: Option<Vec<(String, Token)>>,
    block: Block
}

impl Procedure{
    pub fn new(name:String, params: Option<Vec<(String, Token)>>, block: Block) -> Result<Self, String> {
        for n in &params {
            for var in n {
                if block.contains(&var.0){
                    return Err(
                        better_error(
                            format!("Duplicate identifier `{}` found in Procedure {}.", &var.0, &name), 
                            &var.1
                        )
                    )
                }
            }
        }
        Ok(Self {
            name,
            params,
            block
        })
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }
}