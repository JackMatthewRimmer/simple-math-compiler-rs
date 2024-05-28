use crate::ast::Expr;

pub fn eval(expr: Expr) ->  f64 {
    match expr {
        Expr::Divide(e1, e2) => eval(*e1) / eval(*e2),
        Expr::Multiply(e1, e2)=> eval(*e1) * eval(*e2),
        Expr::Add(e1, e2)=> eval(*e1) + eval(*e2),
        Expr::Subtract(e1, e2)=> eval(*e1) - eval(*e2),
        Expr::Number(num) => num as f64
    }
}
