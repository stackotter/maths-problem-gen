use std::fmt::Display;

pub mod eval;
pub mod gen;

pub struct Pair {
    first: Expr,
    second: Expr,
}

impl Pair {
    pub fn new(first: Expr, second: Expr) -> Self {
        Self { first, second }
    }
}

#[derive(Clone)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: u64,
}

pub enum ExactVal {
    Rational(Rational),
}

pub enum Expr {
    Rational(Rational),
    Add(Box<Pair>),
    Sub(Box<Pair>),
    Mul(Box<Pair>),
    Div(Box<Pair>),
    Negative(Box<Expr>),
}

pub struct Equation {
    pub lhs: Expr,
    pub rhs: Expr,
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator == 1 {
            f.write_str(&format!("{}", self.numerator))
        } else {
            f.write_str(&format!("({} / {})", self.numerator, self.denominator))
        }
    }
}

impl Rational {
    pub fn int(val: i64) -> Self {
        Self {
            numerator: val,
            denominator: 1,
        }
    }

    pub fn new(numerator: i64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl Into<Expr> for Rational {
    fn into(self) -> Expr {
        Expr::Rational(self)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Rational(rational) => f.write_str(&format!("{}", rational)),
            Expr::Add(pair) => f.write_str(&format!("({} + {})", pair.first, pair.second)),
            Expr::Sub(pair) => f.write_str(&format!("({} - {})", pair.first, pair.second)),
            Expr::Mul(pair) => f.write_str(&format!("({} * {})", pair.first, pair.second)),
            Expr::Div(pair) => f.write_str(&format!("({} / {})", pair.first, pair.second)),
            Expr::Negative(expr) => f.write_str(&format!("-{}", expr)),
        }
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} = {}", self.lhs, self.rhs))
    }
}
