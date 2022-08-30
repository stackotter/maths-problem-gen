use std::mem::swap;

use crate::{Expr, Op, Pair, Rational};

fn lcm(a: u64, b: u64) -> u64 {
    let mut guess = if a > b { a } else { b };

    loop {
        if guess % a == 0 && guess % b == 0 {
            return guess;
        }

        guess += 1;
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut max = a;
    let mut min = b;
    if min > max {
        swap(&mut max, &mut min);
    }

    loop {
        let rem = max % min;
        if rem == 0 {
            return min;
        }

        max = min;
        min = rem;
    }
}

impl Rational {
    pub fn simplified(&self) -> Rational {
        let divisor = gcd(self.numerator.abs() as u64, self.denominator);
        if divisor == 1 {
            self.to_owned()
        } else {
            Rational {
                numerator: self.numerator / (divisor as i64),
                denominator: self.denominator / divisor,
            }
        }
    }
}

pub fn eval(expr: &Expr) -> Rational {
    let answer = match expr {
        Expr::Rational(rational) => rational.to_owned(),
        Expr::Pair(pair) => {
            let lval = eval(&pair.left);
            let mut rval = eval(&pair.right);

            match pair.op {
                Op::Add => {
                    let new_denom = lcm(lval.denominator, rval.denominator);
                    let lmult = (new_denom / lval.denominator) as i64;
                    let rmult = (new_denom / rval.denominator) as i64;
                    Rational {
                        numerator: lval.numerator * lmult + rval.numerator * rmult,
                        denominator: new_denom,
                    }
                }
                Op::Sub => {
                    rval.numerator = -rval.numerator;

                    eval(&Expr::Pair(Box::new(Pair::new(
                        Expr::Rational(lval),
                        Op::Add,
                        Expr::Rational(rval),
                    ))))
                }
                Op::Mul => Rational {
                    numerator: lval.numerator * rval.numerator,
                    denominator: lval.denominator * rval.denominator,
                },
                Op::Div => {
                    let denom_sign = rval.numerator.signum();

                    Rational {
                        numerator: lval.numerator * (rval.denominator as i64) * denom_sign,
                        denominator: lval.denominator * (rval.numerator as u64),
                    }
                }
            }
        }
        Expr::Negative(expr) => {
            let mut val = eval(expr);
            val.numerator = -val.numerator;
            val
        }
    };

    answer.simplified()
}
