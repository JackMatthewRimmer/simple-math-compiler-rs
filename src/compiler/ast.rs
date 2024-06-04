use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "compiler/grammar.pest"]
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

enum SumOp {
    Sub,
    Add,
}

enum FactorOp {
    Mul,
    Div,
}

impl From<&str> for SumOp {
    fn from(literal: &str) -> SumOp {
        let trimmed = literal.trim();
        match trimmed {
            "-" => SumOp::Sub,
            "+" => SumOp::Add,
            _ => panic!("SumOp constructed from invalid string"),
        }
    }
}

impl From<&str> for FactorOp {
    fn from(literal: &str) -> FactorOp {
        let trimmed = literal.trim();
        match trimmed {
            "*" => FactorOp::Mul,
            "/" => FactorOp::Div,
            _ => panic!("FactorOp constructed from invalid string"),
        }
    }
}

impl Expr {
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

    pub fn as_sum(pair: Pair<Rule>) -> Expr {
        let mut inner_rules = pair.into_inner();
        let mut expr = Self::as_factor(inner_rules.next().unwrap());

        while let Some(pair) = inner_rules.next() {
            match pair.as_rule() {
                Rule::sumOp => {
                    let op: SumOp = pair.as_str().into();
                    let next_factor = Self::as_factor(inner_rules.next().unwrap());

                    expr = match op {
                        SumOp::Add => Expr::Add(Box::new(expr), Box::new(next_factor)),
                        SumOp::Sub => Expr::Subtract(Box::new(expr), Box::new(next_factor)),
                    }
                }
                _ => unreachable!("unreachable in as_sum"),
            }
        }
        expr
    }

    fn as_factor(pair: Pair<Rule>) -> Expr {
        let mut inner_rules = pair.into_inner();
        let mut expr = Self::as_power(inner_rules.next().unwrap());

        while let Some(pair) = inner_rules.next() {
            match pair.as_rule() {
                Rule::factorOp => {
                    let op: FactorOp = pair.as_str().into();
                    let next_factor = Self::as_power(inner_rules.next().unwrap());

                    expr = match op {
                        FactorOp::Mul => Expr::Multiply(Box::new(expr), Box::new(next_factor)),
                        FactorOp::Div => Expr::Divide(Box::new(expr), Box::new(next_factor)),
                    }
                }
                _ => {
                    dbg!(pair);
                    unreachable!("unreachable in as_factor")
                }
            }
        }
        expr
    }

    fn as_power(pair: Pair<Rule>) -> Expr {
        let mut inner_rules = pair.into_inner();
        let term = Self::as_term(inner_rules.next().unwrap());

        match inner_rules.next() {
            Some(power_pair) => {
                let power = Self::as_power(power_pair);
                Expr::Power(Box::new(term), Box::new(power))
            }
            None => term,
        }
    }

    fn as_term(pair: Pair<Rule>) -> Expr {
        let inner_rule: Pair<Rule> = pair.into_inner().next().expect("no inner rule for factor");
        match inner_rule.as_rule() {
            Rule::number => Self::as_number(inner_rule),
            Rule::sum => Self::as_sum(inner_rule),
            _ => unreachable!("unreachable in as_term"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pairs = ExprParser::parse(Rule::sum, "3 * 2").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 6 as f64);
    }

    #[test]
    fn test2() {
        let pairs = ExprParser::parse(Rule::sum, "7 - 3 - 2").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 2 as f64);
    }

    #[test]
    fn test3() {
        let pairs = ExprParser::parse(Rule::sum, "5 + 4 * 2").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 13 as f64);
    }

    #[test]
    fn test4() {
        let pairs = ExprParser::parse(Rule::sum, "10 / 2 + 3").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 8 as f64);
    }

    #[test]
    fn test5() {
        let pairs = ExprParser::parse(Rule::sum, "2 ^ 3 + 4").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 12 as f64);
    }

    #[test]
    fn test6() {
        let pairs = ExprParser::parse(Rule::sum, "2 * (3 + 4)").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 14 as f64);
    }

    #[test]
    fn test7() {
        let pairs = ExprParser::parse(Rule::sum, "8 / (2 * 2)").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 2 as f64);
    }

    #[test]
    fn test8() {
        let pairs = ExprParser::parse(Rule::sum, "2 ^ (3 + 2)").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 32 as f64);
    }

    #[test]
    fn test9() {
        let pairs = ExprParser::parse(Rule::sum, "2 * 3 - 4").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 2 as f64);
    }

    #[test]
    fn test10() {
        let pairs = ExprParser::parse(Rule::sum, "5 + 4 / 2").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 7 as f64);
    }

    #[test]
    fn test11() {
        let pairs = ExprParser::parse(Rule::sum, "2 ^ 3 - 1").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 7 as f64);
    }

    #[test]
    fn test12() {
        let pairs = ExprParser::parse(Rule::sum, "10 / (2 + 3)").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 2 as f64);
    }

    #[test]
    fn test13() {
        let pairs = ExprParser::parse(Rule::sum, "2 * (3 - 1)").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 4 as f64);
    }

    #[test]
    fn test14() {
        let pairs = ExprParser::parse(Rule::sum, "8 / (2 ^ 2)").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 2 as f64);
    }

    #[test]
    fn test15() {
        let pairs = ExprParser::parse(Rule::sum, "2 ^ (3 - 2)").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 2 as f64);
    }

    #[test]
    fn test16() {
        let pairs = ExprParser::parse(Rule::sum, "(7-3-2) ^ 2").unwrap();
        let pair = Expr::as_sum(pairs.into_iter().next().unwrap());
        assert_eq!(Expr::eval(pair), 4 as f64);
    }
}
