mod utils;

use std::io;
use std::io::Write;
use utils::interpreter::Interpreter;
use utils::parser::Parser;





fn main() {
    let mut expr = String::new();
    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut expr).unwrap();
        if expr.starts_with("debug:") {
            expr.drain(..6);
            let mut parser = Parser::new(expr.trim().as_bytes()).unwrap();
            println!("{:#?}", parser.parse())
        }
        else if expr.starts_with("rewrite: ") {
            expr.drain(..8);
            let interp = Interpreter::new(expr.trim().as_bytes()).unwrap();
            println!("{}",interp.spit().unwrap())
        }
        else {
            let words = expr.trim().as_bytes();
            if words == "exit".as_bytes() {
                break;
            }
            let interp = Interpreter::new(words).unwrap();
            println!("{}", interp.interprete().unwrap());
        }
        expr.clear()
    }
}
