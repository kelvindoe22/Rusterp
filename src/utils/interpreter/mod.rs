use super::lexer:: {Token, Operators};
use super::ast::AST;
use super::parser::Parser;
use std::collections::HashMap;
use std::cell::Cell;

pub struct Interpreter{
    parser: AST<Token>,
    global_scope: Cell<HashMap<String, Token>>
}

impl Interpreter {
    pub fn new<'a>(bytes: &'a [u8]) -> Result<Self, String> {
        Ok(Self {
            parser : Parser::new(bytes)?.program()?,
            global_scope: Cell::new(HashMap::new())
        })
    }


    pub fn interprete(&self) -> Result<(), String> {
        assert!(self.parser.view() == &Token::BEGIN);
        self.visit_begin(&self.parser)
    }

    pub fn print_global_scope(&self) {
        let map = self.global_scope.replace(HashMap::with_capacity(0));
        println!("{:?}", map);
        self.global_scope.replace(map);
    }

    fn visit_begin(&self, begin: &AST<Token>) -> Result<(), String> {
        for i in begin.children() {
            match i.view() {
               Token::ASSIGN => {
                   let string = match i.left().unwrap().view(){
                       Token::IDENTIFIER(e) => {
                           e.clone()
                       }
                       e => return Err(format!("Expected Identifier found {:?}",e))

                   };
                   let value = Token::Integer(self.visit_node(i.right().unwrap())?);
                   let mut map: HashMap<String, Token> = self.global_scope.replace(HashMap::with_capacity(0));
                   map.insert(string, value);
                   self.global_scope.replace(map);
               }

               Token::BEGIN => {
                   self.visit_begin(i)?;
               }

               Token::EMPTY => {}
               
               e => return Err(format!("Expected assignment statement or BEGIN but found {:?}", e))
            }

       }
        Ok(())
    }

    fn visit_node(&self, node: &AST<Token>) -> Result<f64, String>{
        use Operators::*;
        match node.view() {
            Token::Integer(num) => return Ok(*num),
            Token::IDENTIFIER(str) => {
                let yay = self.global_scope.replace(HashMap::with_capacity(0));
                match yay.get(str).unwrap(){
                    Token::Integer(int) => {
                        let res = Ok(*int);
                        self.global_scope.replace(yay);
                        return res;
                    }
                    _ => return Err(format!("Could not find variable {}", str)) 
                }
            }
            Token::Operator(ref op) => {
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
                    
                    DIVISION => return Ok(self.visit_node(node.left().unwrap())? / self.visit_node(node.right().unwrap())?),

                }
            }
            t => { return Err(format!("Expected integer found {:?}",t)) }
        }
    }

    fn rewrite(node: &AST<Token>) -> Result<String, String> {
        use Operators::*;
        match *node.view() {
            Token::Integer(num) => return Ok(format!("{}",num)),
            Token::END => return Ok(format!("End \n")),
            Token::IDENTIFIER(ref s) => return Ok(s.clone()),
            Token::SEMICOLON => return Ok(format!(";")),
            Token::ASSIGN => return Ok(format!("{} := {};", Self::rewrite(node.left().unwrap())?, Self::rewrite(node.right().unwrap())?)),
            Token::DOT => return Ok(format!(".")),
            Token::EMPTY => return Ok(format!("")),
            Token::Operator(ref op) => {
                match (op, node.children().len()){
                    (PLUS,2 )=> return Ok(format!("({} + {})",Self::rewrite(node.left().unwrap())?,Self::rewrite(node.right().unwrap())?)),
                    (PLUS,1)=> return Ok(format!("({})",Self::rewrite(node.left().unwrap())?)),
                    (MINUS,2) => return Ok(format!("({} - {})",Self::rewrite(node.left().unwrap())?,Self::rewrite(node.right().unwrap())?)),
                    (MINUS,1) => return Ok(format!("(-{})",Self::rewrite(node.left().unwrap())?)),
                    (MULTIPLICATION, 2) => return Ok(format!("({} * {})",Self::rewrite(node.left().unwrap())?,Self::rewrite(node.right().unwrap())?)),
                    (DIVISION, 2) => return Ok(format!("({} / {})",Self::rewrite(node.left().unwrap())?,Self::rewrite(node.right().unwrap())?)),
                    _ => unreachable!()
                }
            }
            Token::BEGIN => return {
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
        Self::rewrite(&self.parser)
    }

}

