use simple_math_compiler_rs::compiler::Expr;
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

        let expr = Expr::parse(trim);
        println!("The answer is: {}", Expr::eval(expr));
    }
}
