use crate::{Equation, Expr, Op, Pair, Rational, Func};
use std::fmt::Display;

pub fn bracketize(s: &str) -> String {
    format!("({})", s)
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Pow => "^",
        };
        f.write_str(&c)
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (lrequires_brackets, rrequires_brackets) = self.requires_brackets(false, true);

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

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Func::Sine => "sin",
            Func::Cosine => "cos"
        };
        f.write_str(s)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Rational(rational) => f.write_str(&format!("{}", rational)),
            Expr::Pair(pair) => f.write_str(&format!("{}", pair)),
            Expr::Negative(expr) => f.write_str(&format!("-{}", expr)),
            Expr::Variable(var) => f.write_str(&String::from(var.to_owned())),
            Expr::Derivative(expr) => f.write_str(&format!("ddx({})", expr)),
            Expr::Func(func, inner) => f.write_str(&format!("{func}({inner})")),
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
