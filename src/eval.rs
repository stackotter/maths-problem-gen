use crate::{Expr, Pair, Rational};

fn lcm(a: u64, b: u64) -> u64 {
    let mut guess = if a > b { a } else { b };

    loop {
        if a % guess == 0 && b % guess == 0 {
            return guess;
        }

        guess += 1;
    }
}

pub fn eval(expr: &Expr) -> Rational {
    match expr {
        Expr::Rational(rational) => rational.to_owned(),
        Expr::Add(pair) => {
            let lval = eval(&pair.first);
            let rval = eval(&pair.second);
            let new_denom = lcm(lval.denominator, rval.denominator);
            let lmult = (new_denom / lval.denominator) as i64;
            let rmult = (new_denom / rval.denominator) as i64;
            Rational {
                numerator: lval.numerator * lmult + rval.numerator * rmult,
                denominator: new_denom,
            }
        }
        Expr::Sub(pair) => {
            let lval = eval(&pair.first);
            let mut rval = eval(&pair.second);
            rval.numerator = -rval.numerator;

            eval(&Expr::Add(Box::new(Pair::new(
                Expr::Rational(lval),
                Expr::Rational(rval),
            ))))
        }
        Expr::Mul(pair) => {
            let lval = eval(&pair.first);
            let rval = eval(&pair.second);

            Rational {
                numerator: lval.numerator * rval.numerator,
                denominator: lval.denominator * rval.denominator,
            }
        }
        Expr::Div(pair) => {
            let lval = eval(&pair.first);
            let rval = eval(&pair.second);
            let denom_sign = rval.numerator.signum();

            Rational {
                numerator: lval.numerator * (rval.denominator as i64) * denom_sign,
                denominator: lval.denominator * (rval.numerator as u64),
            }
        }
        Expr::Negative(expr) => {
            let mut val = eval(expr);
            val.numerator = -val.numerator;
            val
        }
    }
}
