use crate::{
    Expr,
    Op::{self, Add, Div, Mul, Sub},
    Pair, Rational,
};

fn int(int: i64) -> Expr {
    Rational::int(int).into()
}

fn pair(l: Expr, op: Op, r: Expr) -> Expr {
    Pair::new(l, op, r).into()
}

pub fn gen_addition() -> Expr {
    Expr::Pair(Box::new(Pair::new(
        Rational::int(8).into(),
        Add,
        Rational::int(3).into(),
    )))
}

pub fn gen_all() -> Expr {
    pair(
        pair(
            int(8),
            Add,
            int(3)
        ),
        Add,
        pair(
            int(1),
            Div,
            pair(
                Rational::new(2, 3).into(),
                Mul,
                Rational::new(6, 2).into()
            )
        )
    )
}
