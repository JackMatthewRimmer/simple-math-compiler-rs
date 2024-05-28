use pest::Parser;
use simple_math_compiler_rs::ast::{Expr, ExprParser, Rule};
use simple_math_compiler_rs::eval::eval;

fn main() {
    let pairs = ExprParser::parse(Rule::subExpr, "6+2^2").unwrap();
    for pair in pairs {
        let expr: Expr = Expr::as_sub_expr(pair);
        println!("The answer is: {}", eval(expr))
    }
}
