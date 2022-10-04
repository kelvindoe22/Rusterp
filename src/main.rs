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
        let words = expr.trim().as_bytes();
        if words == "exit".as_bytes() {
            break;
        }
        let mut interp = Interpreter::new(words).unwrap();
        println!("{}", interp.expr().unwrap());
        expr.clear()
    }
}
