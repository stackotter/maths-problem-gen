use crate::{Expr, Func, Op, Pair, Rational};

pub fn derive(expr: &Expr) -> Expr {
    if expr.unknown_count() == 0 {
        return Rational::int(0).into();
    }

    match expr {
        Expr::Rational(_) => Rational::int(0).into(),
        Expr::Pair(pair) => match pair.op {
            Op::Add | Op::Sub => {
                Pair::new(derive(&pair.left), pair.op.clone(), derive(&pair.right)).into()
            }
            Op::Mul => Pair::new(
                Pair::new(derive(&pair.left), Op::Mul, pair.right.clone()).into(),
                Op::Add,
                Pair::new(pair.left.clone(), Op::Mul, derive(&pair.right)).into(),
            )
            .into(),
            Op::Div => Pair::new(
                Pair::new(
                    Pair::new(derive(&pair.left), Op::Mul, pair.right.clone()).into(),
                    Op::Sub,
                    Pair::new(pair.left.clone(), Op::Mul, derive(&pair.right)).into(),
                )
                .into(),
                Op::Div,
                Pair::new(pair.right.clone(), Op::Pow, Rational::int(2).into()).into(),
            )
            .into(),
            Op::Pow => {
                if pair.right.unknown_count() == 0 {
                    Pair::new(
                        Pair::new(
                            pair.right.clone(),
                            Op::Mul,
                            Pair::new(
                                pair.left.clone(),
                                Op::Pow,
                                Pair::new(pair.right.clone(), Op::Sub, Rational::int(1).into())
                                    .into(),
                            )
                            .into(),
                        )
                        .into(),
                        Op::Mul,
                        derive(&pair.left),
                    )
                    .into()
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
