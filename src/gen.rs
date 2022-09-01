use rand::{self, Rng};

use crate::{simplify::simplify, Equation, Expr, Op, Pair, Rational};

fn pair(l: Expr, op: Op, r: Expr) -> Expr {
    Pair::new(l, op, r).into()
}

pub fn rand_int() -> Rational {
    let mut rng = rand::thread_rng();
    let n: i64 = rng.gen_range(1..10);
    Rational::int(n)
}

pub fn rand_rational() -> Rational {
    let mut rng = rand::thread_rng();
    let numerator: i64 = rng.gen_range(1..35);
    let denominator: u64 = rng.gen_range(1..10);
    Rational::new(numerator, denominator)
}

pub fn factors(n: Rational) -> Vec<Rational> {
    if n.denominator != 1 {
        return vec![];
    }

    let numerators = (1..(n.numerator + 1))
        .into_iter()
        .filter(|&x| n.numerator % x == 0)
        .collect::<Vec<i64>>();
    numerators
        .into_iter()
        .map(|numerator| Rational::int(numerator))
        .collect()
}

pub fn gen<Rand: Fn() -> Rational>(
    depth: u64,
    answer: Rational,
    rand_term: &Rand,
    previous_op: Option<Op>,
) -> Expr {
    if depth == 0 {
        answer.into()
    } else {
        let mut rng = rand::thread_rng();
        let mut ops = vec![Op::Add, Op::Sub, Op::Mul, Op::Div];
        if let Some(previous_op) = previous_op {
            ops = ops.into_iter().filter(|&op| op != previous_op).collect();
        }
        let mut op = ops[rng.gen_range(0..(ops.len()))];
        let l = match op {
            Op::Mul => {
                let nice_numbers: Vec<Rational> = factors(answer)
                    .into_iter()
                    .filter(|&factor| factor != Rational::int(1) && factor != answer)
                    .collect();
                if nice_numbers.len() == 0 {
                    return gen(depth, answer, rand_term, previous_op);
                }
                nice_numbers[rng.gen_range(0..(nice_numbers.len()))]
            }
            Op::Add | Op::Sub => {
                let mut l = rand_term();
                if l >= answer {
                    l = l + Rational::int(1);
                    op = Op::Sub;
                } else {
                    op = Op::Add;
                }
                l
            }
            Op::Div => rand_term() * answer,
            Op::Pow => unreachable!(),
        };

        let r = match op {
            Op::Add => answer - l,
            Op::Sub => l - answer,
            Op::Mul => answer / l,
            Op::Div => l / answer,
            Op::Pow => unreachable!(),
        };
        let lexpr = gen(depth - 1, l, rand_term, Some(op));
        let rexpr = gen(depth - 1, r, rand_term, Some(op));
        let value = pair(lexpr, op, rexpr);
        value
    }
}

pub fn gen_arithmetic(depth: u64, answer: Rational) -> Expr {
    gen(depth, answer, &|| rand_int(), None)
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
    let mut lhs = gen(depth, rhs, &|| rand_int(), None);

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

pub fn gen_choices(answer: Rational, count: usize) -> Vec<Rational> {
    let mut answers = vec![];

    let mut offsets: Vec<_> = (-4..4).collect();

    for _ in 0..count {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..offsets.len());
        let offset = offsets.remove(index);
        answers.push(answer + Rational::int(offset));
    }

    answers
}

pub fn gen_polynomial(degree: u64) -> Expr {
    let mut rng = rand::thread_rng();
    let mut pair = Pair::new(Rational::int(0).into(), Op::Add, Rational::int(0).into());
    for exponent in (0..=degree).rev() {
        let coefficient = rng.gen_range(-10..10);
        if coefficient == 0 {
            continue;
        }

        pair = Pair::new(
            pair.into(),
            Op::Add,
            Pair::new(
                Rational::int(coefficient).into(),
                Op::Mul,
                Pair::new(
                    Expr::Variable('x'),
                    Op::Pow,
                    Rational::int(exponent as i64).into(),
                )
                .into(),
            )
            .into(),
        );
    }

    simplify(&pair.into())
}
