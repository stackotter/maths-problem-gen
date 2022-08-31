use crate::{Rational, Equation, Expr, eval::{eval, EvalErr}, Op, Pair};

#[derive(Debug)]
pub enum SolveErr {
    TooManyUnknowns,
    NoUnknowns,
    FailedToEval(EvalErr)
}

impl Expr {
    fn unknown_count(&self) -> u64 {
        match self {
            Expr::Rational(_) => 0,
            Expr::Pair(pair) => pair.left.unknown_count() + pair.right.unknown_count(),
            Expr::Negative(expr) => expr.unknown_count(),
            Expr::Variable(_) => 1
        }
    }
}

pub fn solve(equation: &Equation) -> Result<Rational, SolveErr> {
    let lunknowns = equation.lhs.unknown_count();
    let runknowns = equation.rhs.unknown_count();

    let unknowns = lunknowns + runknowns;

    if unknowns > 1 {
        return Err(SolveErr::TooManyUnknowns)
    } else if unknowns == 0 {
        return Err(SolveErr::NoUnknowns)
    }

    let mut side_with_unknown;
    let mut constant_side;
    if lunknowns == 1 {
        side_with_unknown = equation.lhs.clone();
        constant_side = equation.rhs.clone();
    } else {
        side_with_unknown = equation.rhs.clone();
        constant_side = equation.lhs.clone();
    }

    loop {
        match side_with_unknown {
            Expr::Rational(_) => panic!("Unknown disappeared"),
            Expr::Negative(expr) => {
                side_with_unknown = *expr;
                constant_side = Expr::Negative(Box::new(constant_side));
            }
            Expr::Variable(_) => {
                let answer = eval(&constant_side).map_err(|e| SolveErr::FailedToEval(e))?;
                return Ok(answer);
            }
            Expr::Pair(pair) => {
                let lunknowns = pair.left.unknown_count();
                if lunknowns == 1 {
                    side_with_unknown = pair.left;
                    constant_side = Expr::Pair(Box::new(Pair::new(constant_side, pair.op.inverse(), pair.right)));
                } else {
                    side_with_unknown = pair.right.clone();
                    match pair.op {
                        Op::Add | Op::Mul => constant_side = Expr::Pair(Box::new(Pair::new(constant_side, pair.op.inverse(), pair.left))),
                        Op::Div | Op::Sub => constant_side = Expr::Pair(Box::new(Pair::new(pair.left, pair.op, constant_side)))
                    }
                }
            }
        }
    }
}
