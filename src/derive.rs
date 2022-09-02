use crate::{Expr, Func, Op, Pair, Rational};

pub fn derive(expr: &Expr) -> Expr {
    match expr {
        Expr::Rational(_) => Rational::int(0).into(),
        Expr::Pair(pair) => match pair.op {
            Op::Add | Op::Sub => Expr::Pair(Box::new(Pair::new(
                derive(&pair.left),
                pair.op.clone(),
                derive(&pair.right),
            ))),
            Op::Mul => Expr::Pair(Box::new(Pair::new(
                Expr::Pair(Box::new(Pair::new(
                    derive(&pair.left),
                    Op::Mul,
                    pair.right.clone(),
                ))),
                Op::Add,
                Expr::Pair(Box::new(Pair::new(
                    pair.left.clone(),
                    Op::Mul,
                    derive(&pair.right),
                ))),
            ))),
            Op::Div => Expr::Pair(Box::new(Pair::new(
                Expr::Pair(Box::new(Pair::new(
                    Expr::Pair(Box::new(Pair::new(
                        derive(&pair.left),
                        Op::Mul,
                        pair.right.clone(),
                    ))),
                    Op::Sub,
                    Expr::Pair(Box::new(Pair::new(
                        pair.left.clone(),
                        Op::Mul,
                        derive(&pair.right),
                    ))),
                ))),
                Op::Div,
                Expr::Pair(Box::new(Pair::new(
                    pair.right.clone(),
                    Op::Mul,
                    pair.right.clone(),
                ))),
            ))),
            Op::Pow => {
                if pair.right.unknown_count() == 0 {
                    Expr::Pair(Box::new(Pair::new(
                        pair.right.clone(),
                        Op::Mul,
                        Expr::Pair(Box::new(Pair::new(
                            pair.left.clone(),
                            Op::Pow,
                            Expr::Pair(Box::new(Pair::new(
                                pair.right.clone(),
                                Op::Sub,
                                Rational::int(1).into(),
                            ))),
                        ))),
                    )))
                } else {
                    unimplemented!();
                }
            }
        },
        Expr::Negative(expr) => Expr::Negative(Box::new(derive(&expr))),
        Expr::Variable(_) => Rational::int(1).into(),
        Expr::Derivative(expr) => derive(&derive(&expr)),
        Expr::Func(func, inner) => Pair::new(
            match func {
                Func::Sine => Expr::Func(Func::Cosine, inner.to_owned()),
                Func::Cosine => Expr::Negative(Box::new(Expr::Func(Func::Sine, inner.to_owned()))),
            },
            Op::Mul,
            derive(&*inner),
        )
        .into(),
    }
}
