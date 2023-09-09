mod utils;

use utils::interpreter::Interpreter;
use utils::lexer::Lexer;
use std::fs::File;
use std::io::prelude::*;

use crate::utils::lexer::TokenType;



fn main() {
    let  mut args = std::env::args().skip(1);
    let next = args.next();
    let mut string = String::new();
    let mut file = File::open("test.pa").unwrap();
    file.read_to_string(&mut string).unwrap();
    match next {
        Some(s) => {
            match s.as_str() {
                "rewrite" => {
                    println!("{}",Interpreter::new(string.as_bytes()).unwrap().spit().unwrap())
                }
                "ast" => {
                    let mut lex = Lexer::new(string.as_bytes());
                    let mut token = lex.get_next_token().unwrap();
                    loop {
                        println!("{:?}", token);
                        if let TokenType::EOF = token.token_type() {
                            break;
                        }
                        token = lex.get_next_token().unwrap();
                    }
                }
                "genscope" =>  {
                    let interp = Interpreter::new(string.as_bytes()).unwrap();
                    interp.interprete().unwrap();
                    interp.print_global_scope();
                },
                _ => {
                    println!("{}",s);
                }
            }
        }
        None => {todo!()}
    }
}


