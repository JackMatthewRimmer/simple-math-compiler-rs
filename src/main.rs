use pest::Parser;
use simple_math_compiler_rs::ast::{Expr, ExprParser, Rule};
use simple_math_compiler_rs::eval::eval;

fn main() {
    let pairs = ExprParser::parse(Rule::expr, "10+2*3-2").unwrap();
    for pair in pairs {
        let expr: Expr = Expr::as_expr(pair);
        println!("The answer is: {}", eval(expr))
    }
}
