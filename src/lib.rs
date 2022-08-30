use std::char;

pub mod eval;
pub mod fmt;
pub mod gen;
pub mod render;
pub mod solve;
pub mod derive;

#[derive(PartialEq, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn precedence(&self) -> u64 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div => 2,
        }
    }

    pub fn is_associative(&self) -> bool {
        match self {
            Op::Add | Op::Mul => true,
            Op::Sub | Op::Div => false,
        }
    }

    pub fn inverse(&self) -> Op {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul
        }
    }
}

#[derive(Clone)]
pub struct Pair {
    pub left: Expr,
    pub right: Expr,
    pub op: Op,
}

impl Pair {
    pub fn new(left: Expr, op: Op, right: Expr) -> Self {
        Self { left, op, right }
    }

    pub fn requires_brackets(&self, division_as_fraction: bool) -> (bool, bool) {
        if self.op == Op::Div && division_as_fraction {
            return (false, false);
        }

        let lprecedence = self.left.precedence();
        let precedence = self.op.precedence();
        let rprecedence = self.right.precedence();

        let mut lrequires = lprecedence < precedence;
        let mut rrequires =
            rprecedence < precedence || (rprecedence == precedence && self.op.is_associative());

        if division_as_fraction {
            match &self.left {
                Expr::Rational(_) => lrequires = false,
                Expr::Pair(pair) => {
                    if pair.op == Op::Div {
                        lrequires = false
                    }
                }
                _ => (),
            }
            match &self.right {
                Expr::Rational(_) => rrequires = false,
                Expr::Pair(pair) => {
                    if pair.op == Op::Div {
                        rrequires = false
                    }
                }
                _ => (),
            }
        }

        (lrequires, rrequires)
    }
}

impl Into<Expr> for Pair {
    fn into(self) -> Expr {
        Expr::Pair(Box::new(self))
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

#[derive(Clone)]
pub enum Expr {
    Rational(Rational),
    Pair(Box<Pair>),
    Negative(Box<Expr>),
    Variable(char),
}

impl Expr {
    pub fn precedence(&self) -> u64 {
        match self {
            Expr::Rational(_) => 2,
            Expr::Pair(pair) => pair.op.precedence(),
            Expr::Negative(_) => 1,
            Expr::Variable(_) => 3
        }
    }
}

pub struct Equation {
    pub lhs: Expr,
    pub rhs: Expr,
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
