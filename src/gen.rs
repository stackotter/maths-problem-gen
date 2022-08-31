use rand::{self, Rng};

use crate::Equation;
use crate::{
    Expr,
    Op::{self, Add, Div, Mul},
    Pair, Rational,
};

fn int(int: i64) -> Expr {
    Rational::int(int).into()
}

fn pair(l: Expr, op: Op, r: Expr) -> Expr {
    Pair::new(l, op, r).into()
}

fn expr_add(l: Expr, r: Expr) -> Expr {
    pair(l.into(), Add, r.into())
}

pub fn rand_int() -> Rational {
    let mut rng = rand::thread_rng();
    let n: i64 = rng.gen_range(1..35);
    Rational::int(n)
}

pub fn rand_rational() -> Rational {
    let mut rng = rand::thread_rng();
    let numerator: i64 = rng.gen_range(1..35);
    let denominator: u64 = rng.gen_range(1..10);
    Rational::new(numerator, denominator)
}

pub fn gen_add<F: Fn() -> Expr>(term_count: u64, rand_term: F) -> Expr {
    let l = rand_term();
    if term_count == 1 {
        l
    } else {
        let r = gen_add(term_count - 1, rand_term);
        expr_add(l, r)
    }
}

pub fn gen_simple_add(term_count: u64) -> Expr {
    gen_add(term_count, || rand_int().into())
}

pub fn gen_all() -> Expr {
    pair(
        pair(int(8), Add, int(3)),
        Add,
        pair(
            int(1),
            Div,
            pair(Rational::new(2, 3).into(), Mul, Rational::new(6, 2).into()),
        ),
    )
}

pub fn gen_equation() -> Equation {
    Equation {
        lhs: pair(
            pair(Expr::Variable('x'), Add, int(3)),
            Add,
            pair(
                int(1),
                Div,
                pair(Rational::new(2, 3).into(), Mul, Rational::new(6, 2).into()),
            ),
        ),
        rhs: Expr::Rational(Rational::new(23, 2)),
    }
}
