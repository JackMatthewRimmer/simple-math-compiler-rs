use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ExprParser {}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn as_sub_expr(pair: Pair<Rule>) -> Expr {
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
