

pub mod functions {
    use crate::utils::lexer::Token;

    pub fn better_error(str: String, t: &Token) -> String{
        format!("{} Postion line_no:{} column: {}", str, t.line_no(), t.column())
    }
}