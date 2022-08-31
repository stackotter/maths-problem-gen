use rand::{self, Rng};

use crate::{Equation, Expr, Op, Pair, Rational};

fn pair(l: Expr, op: Op, r: Expr) -> Expr {
    Pair::new(l, op, r).into()
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

pub fn gen<Rand: Fn() -> Rational>(depth: u64, answer: Rational, rand_term: &Rand) -> Expr {
    if depth == 0 {
        answer.into()
    } else {
        let l = rand_term();
        let mut rng = rand::thread_rng();
        let op = [Op::Add, Op::Sub, Op::Mul, Op::Div][rng.gen_range(0..4)];
        let r = match op {
            Op::Add => answer - l,
            Op::Sub => l - answer,
            Op::Mul => answer / l,
            Op::Div => l / answer,
        };
        let lexpr = gen(depth - 1, l, rand_term);
        let rexpr = gen(depth - 1, r, rand_term);
        let value = pair(lexpr, op, rexpr);
        value
    }
}

pub fn gen_arithmetic(depth: u64, answer: Rational) -> Expr {
    gen(depth, answer, &|| rand_int())
}

#[derive(Debug)]
enum GenErr {
    UnexpectedVariable,
}

fn replace_random_constant(expr: &mut Expr, replacement: Expr) -> Result<Rational, GenErr> {
    match expr {
        Expr::Rational(rational) => {
            let rational = rational.clone();
            *expr = replacement;
            Ok(rational.to_owned())
        }
        Expr::Negative(inner) => replace_random_constant(&mut *inner, replacement),
        Expr::Variable(_) => Err(GenErr::UnexpectedVariable),
        Expr::Pair(pair) => {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.5) {
                replace_random_constant(&mut pair.left, replacement)
            } else {
                replace_random_constant(&mut pair.right, replacement)
            }
        }
    }
}

pub fn gen_backtrack(depth: u64) -> (Equation, Rational) {
    let rhs = rand_int();
    let mut lhs = gen(depth, rhs, &|| rand_int());

    let replaced_term = replace_random_constant(&mut lhs, Expr::Variable('x'))
        .expect("Generated expr shouldn't contain variable yet");

    (
        Equation {
            lhs,
            rhs: rhs.into(),
        },
        replaced_term,
    )
}
