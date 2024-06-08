use pest::Parser;
use simple_math_compiler_rs::compiler::{Expr, ExprParser, Rule};
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

        let pairs = ExprParser::parse(Rule::sum, trim).unwrap();
        let expr = Expr::as_sum(pairs.into_iter().next().unwrap());
        println!("The answer is: {}", Expr::eval(expr));
    }
}
