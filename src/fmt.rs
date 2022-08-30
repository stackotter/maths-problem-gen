use std::fmt::Display;
use crate::{Equation, Expr, Rational, Pair, Op};

pub fn bracketize(s: &str) -> String {
    format!("({})", s)
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/"
        };
        f.write_str(&c)
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (lrequires_brackets, rrequires_brackets) = self.requires_brackets(false);

        let mut l = format!("{}", self.left);
        let mut r = format!("{}", self.right);

        if lrequires_brackets {
            l = bracketize(&l);
        }
        if rrequires_brackets {
            r = bracketize(&r);
        }

        f.write_str(&format!("{} {} {}", l, self.op, r))
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Rational(rational) => f.write_str(&format!("{}", rational)),
            Expr::Pair(pair) => f.write_str(&format!("{}", pair)),
            Expr::Negative(expr) => f.write_str(&format!("-{}", expr)),
        }
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} = {}", self.lhs, self.rhs))
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator == 1 {
            f.write_str(&format!("{}", self.numerator))
        } else {
            f.write_str(&format!("{} / {}", self.numerator, self.denominator))
        }
    }
}
