use crate::{Expr, Pair, Rational};

pub fn gen_addition() -> Expr {
    Expr::Add(Box::new(Pair::new(
        Expr::Rational(Rational {
            numerator: 5,
            denominator: 1,
        }),
        Expr::Rational(Rational {
            numerator: 3,
            denominator: 1,
        }),
    )))
}
