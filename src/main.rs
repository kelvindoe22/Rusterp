mod utils;

use std::io;
use std::io::Write;
use utils::interpreter::Interpreter;

fn main() {
    let mut expr = String::new();
    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut expr).unwrap();
        if expr.starts_with("debug:") {
            expr.drain(..6);
            let interp = Interpreter::new(expr.trim().as_bytes()).unwrap();
            for t in interp.into_iter() {
                println!("{:?}", t);
            }
        } else {
            let words = expr.trim().as_bytes();
            if words == "exit".as_bytes() {
                break;
            }
            let mut interp = Interpreter::new(words).unwrap();
            println!("{}", interp.expr().unwrap());
        }
        expr.clear()
    }
}
