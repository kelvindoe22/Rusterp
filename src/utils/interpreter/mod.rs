use super::ast::program::Program;
use super::err::functions::better_error;
use super::lexer:: {Token, TokenType, Operators};
use super::ast::ast::AST;
use super::parser::Parser;

pub struct Interpreter{
    program: Program,
}

impl Interpreter {
    pub fn new<'a>(bytes: &'a [u8]) -> Result<Self, String> {
        Ok(Self {
            program : Parser::new(bytes)?.program()?
        })
    }

    pub fn interprete(&self) -> Result<(), String> {
        self.visit_begin(&self.program.statements())
    }

    pub fn print_global_scope(&self) {
        self.program.scope()
    }


    fn visit_begin(&self, begin: &AST<Token>) -> Result<(), String> {
        if let TokenType::BEGIN = begin.view().token_type() {
        } else {
            return Err(
                    better_error(
                    format!("Expected BEGIN found {:?}",begin),
                    begin.view()
                    )
            );
        }
        for i in begin.children() {
            match i.view().token_type() {
               TokenType::ASSIGN => {
                    match i.left().unwrap().view().token_type(){
                       TokenType::IDENTIFIER(_) => {
                
                           self.program.set_var(i.left().unwrap().view(), self.visit_node(i.right().unwrap())?)?
                        } 
                           
                       e => return Err(
                            better_error(
                                format!("Expected Identifier found {:?}",e),
                                i.view()
                            )
                       )

                   };
                   
               }

               TokenType::BEGIN => {
                   self.visit_begin(i)?;
               }


               TokenType::PROCEDURE(p) => {
                    match p {
                        Some(_) => {
                            self.program.set_proc(i.view().clone())?;
                        }
                        None => unreachable!()
                    }
               }

               TokenType::EMPTY => {}
               
               _ => {self.visit_node(i)?;},
            }

       }
        Ok(())
    }

    fn visit_node(&self, node: &AST<Token>) -> Result<f64, String>{
        use Operators::*;
        match node.view().token_type() {
            TokenType::Integer(num) => return Ok(num.clone()),
            TokenType::IDENTIFIER(str) => {
                match self.program.get_num(&str){
                    Some(int) => {
                        let res = Ok(int);
                        
                        return res;
                    }
                    None => return Err(format!("Variable `{}` not found.", str)),
                     
                }
            }
            TokenType::Operator(ref op) => {
                match *op{
                    PLUS => {
                        if node.children().len() == 1 {
                            return Ok(self.visit_node(node.left().unwrap())?)
                        } else {
                            return Ok(self.visit_node(node.left().unwrap())? + self.visit_node(node.right().unwrap())?)
                        }
                    }
            
                    MINUS => {
                        if node.children().len() == 1 {
                            return Ok(-self.visit_node(node.left().unwrap())?);
                        } else { 
                            
                            return Ok(self.visit_node(node.left().unwrap())? - self.visit_node(node.right().unwrap())?)
                        }
                    }
                    
                    MULTIPLICATION => return Ok(self.visit_node(node.left().unwrap())? * self.visit_node(node.right().unwrap())?),
                    
                    IDIVISION => return Ok((self.visit_node(node.left().unwrap())? / self.visit_node(node.right().unwrap())?).floor()),
                    FDIVISION => return Ok(self.visit_node(node.left().unwrap())? / self.visit_node(node.right().unwrap())?),

                }
            }
            TokenType::PROCEDURECALL(proc) => {
                let mut params_simplified = Vec::with_capacity(0);
                if proc.params.is_some() {
                    unsafe{
                        params_simplified.reserve(proc.params.as_ref().unwrap_unchecked().len());
                        for i in proc.params.as_ref().unwrap_unchecked() {
                            params_simplified.push(self.visit_node(i)?)
                        }
                    }
                }
                todo!()
            }
            t => { return Err(format!("Cannot interprete token: {:?}.",t)) }
        }
    }

    fn rewrite(node: &AST<Token>) -> Result<String, String> {
        use Operators::*;
        match node.view().token_type() {
            TokenType::Integer(num) => return Ok(format!("{}",num)),
            TokenType::END => return Ok(format!("END \n")),
            TokenType::IDENTIFIER(ref s) => return Ok(s.clone()),
            TokenType::SEMICOLON => return Ok(format!(";")),
            TokenType::ASSIGN => return Ok(format!("{} := {};", Self::rewrite(node.left().unwrap())?, Self::rewrite(node.right().unwrap())?)),
            TokenType::DOT => return Ok(format!(".")),
            TokenType::EMPTY => return Ok(format!("")),
            TokenType::Operator(ref op) => {
                match (op, node.children().len()){
                    (PLUS,2 )=> return Ok(format!("({} + {})",Self::rewrite(node.left().unwrap())?,Self::rewrite(node.right().unwrap())?)),
                    (PLUS,1)=> return Ok(format!("({})",Self::rewrite(node.left().unwrap())?)),
                    (MINUS,2) => return Ok(format!("({} - {})",Self::rewrite(node.left().unwrap())?,Self::rewrite(node.right().unwrap())?)),
                    (MINUS,1) => return Ok(format!("(-{})",Self::rewrite(node.left().unwrap())?)),
                    (MULTIPLICATION, 2) => return Ok(format!("({} * {})",Self::rewrite(node.left().unwrap())?,Self::rewrite(node.right().unwrap())?)),
                    (IDIVISION, 2) => return Ok(format!("({} DIV {})",Self::rewrite(node.left().unwrap())?,Self::rewrite(node.right().unwrap())?)),
                    (FDIVISION, 2) => return Ok(format!("({} / {})",Self::rewrite(node.left().unwrap())?,Self::rewrite(node.right().unwrap())?)),
                    _ => unreachable!()
                }
            }
            TokenType::BEGIN => return {
                let mut res = String::from("BEGIN\n");
                for i in node.children() {
                    res.push_str(&*Self::rewrite(i)?);
                    res.push('\n');
                }
                res.push_str("END");
                Ok(res)
            },
            _ => { unreachable!() }
        }
    }

    pub fn spit(&self) -> Result<String, String>{
        Self::rewrite(&self.program.statements())
    }

}

