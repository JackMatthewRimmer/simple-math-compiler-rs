use pest::Parser;
use simple_math_compiler_rs::ast::{Expr, ExprParser, Rule};
use simple_math_compiler_rs::eval::eval;
use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter an expression (or 'q' to quit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trim = input.trim();
        if trim.eq_ignore_ascii_case("q") {
            break;
        }

        match ExprParser::parse(Rule::subExpr, trim) {
            Ok(pairs) => {
                for pair in pairs {
                    let expr: Expr = Expr::as_sub_expr(pair);
                    println!("The answer is: {}", eval(expr));
                }
            }
            Err(e) => {
                println!("Error parsing expression: {}", e);
            }
        }
    }
}
