use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

/// # ExprParser
///
/// via pest_derive this generates the parser based
/// of our defined grammer.
///
/// # Notes on the grammar
///
/// The way our grammar works is that we have the lowest
/// precdence operators at the top of the grammar. The
/// reasoning for this is when we parse an expression
/// it means the precedence is reflected in the parse tree
/// and as we evaluate recursively anything that is lower
/// in the parse tree will be evaluated first.
///
/// # Example
/// 3 - 1 + 2
///
/// Will generate the AST...
///
/// Subtract(
///     Number(3),
///     Add(
///         Number(1),
///         Number(2)
///     )
/// )
///
/// Which then when we evaluate recursively
/// the Add is evaluated first and then the
/// Subtract is evaluted.
#[derive(Parser)]
#[grammar = "compiler/grammar.pest"]
struct ExprParser {}

/// # Expr
/// This enum is our AST for our math expressions.
/// It represents a math expression as a recursive data
/// structure where an operation can contain sub expressions.
/// which is either a Number or another operation.
#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
}

/// In the impl block for expression we have some code
/// to convert ['Pair'] which is pests type the parsing
/// spits out into our ['Expr'] AST. We can then easily
/// evaluate the AST with the super power of recursion.
impl Expr {
    pub fn parse(input: &str) -> Expr {
        let pair = ExprParser::parse(Rule::subExpr, input)
            .unwrap()
            .next()
            .unwrap();
        Self::as_sub_expr(pair)
    }

    pub fn eval(expr: Expr) -> f64 {
        match expr {
            Expr::Divide(e1, e2) => Self::eval(*e1) / Self::eval(*e2),
            Expr::Multiply(e1, e2) => Self::eval(*e1) * Self::eval(*e2),
            Expr::Add(e1, e2) => Self::eval(*e1) + Self::eval(*e2),
            Expr::Subtract(e1, e2) => Self::eval(*e1) - Self::eval(*e2),
            Expr::Power(e1, e2) => Self::eval(*e1).powf(Self::eval(*e2)),
            Expr::Number(num) => num as f64,
        }
    }

    fn as_sub_expr(pair: Pair<Rule>) -> Expr {
        let mut inner_rules: Pairs<Rule> = pair.clone().into_inner();
        let add_pair = inner_rules.next().expect("No add_pair provided");
        let add_expr: Expr = Self::as_add_expr(add_pair.clone());
        match inner_rules.next() {
            Some(expr_pair) => {
                let expr: Expr = Self::as_sub_expr(expr_pair.clone());
                Expr::Subtract(Box::new(add_expr), Box::new(expr))
            }
            None => add_expr,
        }
    }

    fn as_add_expr(pair: Pair<Rule>) -> Expr {
        let mut inner_rules: Pairs<Rule> = pair.clone().into_inner();
        let mul_pair = inner_rules.next().expect("No mul_pair provided");
        let mul_expr: Expr = Self::as_mul_expr(mul_pair.clone());
        match inner_rules.next() {
            Some(add_pair) => {
                let add_expr: Expr = Self::as_add_expr(add_pair.clone());
                Expr::Add(Box::new(mul_expr), Box::new(add_expr))
            }
            None => mul_expr,
        }
    }

    fn as_mul_expr(pair: Pair<Rule>) -> Expr {
        let mut inner_rules: Pairs<Rule> = pair.clone().into_inner();
        let div_pair = inner_rules.next().expect("No addExpr pair provided");
        let div_expr: Expr = Self::as_div_expr(div_pair.clone());
        match inner_rules.next() {
            Some(mul_pair) => {
                let mul_expr: Expr = Self::as_mul_expr(mul_pair.clone());
                Expr::Multiply(Box::new(div_expr), Box::new(mul_expr))
            }
            None => div_expr,
        }
    }

    fn as_div_expr(pair: Pair<Rule>) -> Expr {
        let mut inner_rules: Pairs<Rule> = pair.clone().into_inner();
        let pow_pair = inner_rules.next().expect("No addExpr pair provided");
        let pow: Expr = Self::as_pow_expr(pow_pair.clone());
        match inner_rules.next() {
            Some(div_pair) => {
                let div_expr: Expr = Self::as_div_expr(div_pair.clone());
                Expr::Divide(Box::new(pow), Box::new(div_expr))
            }
            None => pow,
        }
    }

    fn as_pow_expr(pair: Pair<Rule>) -> Expr {
        let mut inner_rules: Pairs<Rule> = pair.clone().into_inner();
        let factor_pair = inner_rules.next().expect("No addExpr pair provided");
        let factor: Expr = Self::as_factor(factor_pair.clone());
        match inner_rules.next() {
            Some(pow_pair) => {
                let pow_expr: Expr = Self::as_pow_expr(pow_pair.clone());
                Expr::Power(Box::new(factor), Box::new(pow_expr))
            }
            None => factor,
        }
    }

    fn as_factor(pair: Pair<Rule>) -> Expr {
        let inner_rule: Pair<Rule> = pair.into_inner().next().expect("no inner rule for factor");
        match inner_rule.as_rule() {
            Rule::number => Self::as_number(inner_rule),
            Rule::subExpr => Self::as_sub_expr(inner_rule),
            _ => unreachable!(),
        }
    }

    fn as_number(pair: Pair<Rule>) -> Expr {
        assert_eq!(
            pair.as_rule(),
            Rule::number,
            "as_number() called with non digit rule"
        );
        Expr::Number(
            pair.as_str()
                .trim()
                .parse::<i32>()
                .expect("Failed to parse int"),
        )
    }
}
