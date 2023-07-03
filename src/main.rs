mod utils;

use utils::interpreter::Interpreter;
use utils::parser::Parser;
use std::fs::File;
use std::io::prelude::*;



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
                    println!("{:#?}", Parser::new(string.as_bytes()).unwrap().program().unwrap())
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

    // let interp = Interpreter::new(string.as_bytes()).unwrap();
    // interp.interprete().unwrap();
    // interp.print_global_scope();
}


// fn main() {
//     let mut expr = String::new();
//     loop {
//         print!("calc> ");
//         io::stdout().flush().unwrap();
//         std::io::stdin().read_line(&mut expr).unwrap();
//         if expr.starts_with("debug:") {
//             expr.drain(..6);
//             let parser = Parser::new(expr.trim().as_bytes());
//             if let Ok(p) = parser {
//                 let mut a = p.into_iter();
//                 while let Some(item) = a.next() {
//                     println!("{:?}", item)
//                 }
//             } else {
//                 println!("{}",parser.err().unwrap())
//             }
//         }
//         else if expr.starts_with("rewrite: ") {
//             expr.drain(..8);
//             let interp = Interpreter::new(expr.trim().as_bytes());
//             if let Ok(interpreter) = interp {
//                 match interpreter.spit() {
//                     Ok(res) => println!("{}", res),
//                     Err(e) => println!("{}", e)
//                 }
//             } else {
//                 println!("{}", interp.err().unwrap())
//             }
//         }
//         else {
//             let words = expr.trim().as_bytes();
//             if words == "exit".as_bytes() {
//                 break;
//             }
//             match Interpreter::new(words) {
//                 Ok(interp) =>  {
//                     match interp.interprete() {
//                         Ok(res) => println!("{}", res),
//                         Err(e) => println!("{}", e)
//                     }
//                 }
//                 Err(e) => println!("{}", e)
//             }
//         }
//         expr.clear()
//     }
// }
