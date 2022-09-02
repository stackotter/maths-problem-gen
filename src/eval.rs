use std::{
    mem::swap,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{derive::derive, Expr, Func, Op, Rational};

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
        if self.numerator == 0 {
            return Rational::new(0, 1);
        }

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

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self::Output {
        Rational {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
        .simplified()
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        let new_denom = lcm(self.denominator, rhs.denominator);
        let lmult = (new_denom / self.denominator) as i64;
        let rmult = (new_denom / rhs.denominator) as i64;
        Rational {
            numerator: self.numerator * lmult + rhs.numerator * rmult,
            denominator: new_denom,
        }
        .simplified()
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        Rational {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, rhs: Self) -> Self::Output {
        let denom_sign = rhs.numerator.signum();

        Rational {
            numerator: self.numerator * (rhs.denominator as i64) * denom_sign,
            denominator: self.denominator * (rhs.numerator.abs() as u64),
        }
        .simplified()
    }
}

impl Rational {
    fn pow(self, exponent: Rational) -> Rational {
        if exponent.denominator != 1 {
            unimplemented!("Fractional exponents not yet implemented");
        }

        if exponent.numerator == 0 {
            return Rational::int(1);
        }

        let mut result = self;
        for _ in 0..exponent.numerator.abs() {
            result = result * exponent;
        }

        if exponent.numerator > 0 {
            result
        } else {
            Rational::int(1) / result
        }
    }
}

#[derive(Debug)]
pub enum EvalErr {
    EncounteredUnknown(char),
    NonEvaluableFunc(Func),
}

pub fn eval(expr: &Expr) -> Result<Rational, EvalErr> {
    let answer = match expr {
        Expr::Rational(rational) => rational.to_owned(),
        Expr::Pair(pair) => {
            let lval = eval(&pair.left)?;
            let rval = eval(&pair.right)?;

            match pair.op {
                Op::Add => lval + rval,
                Op::Sub => lval - rval,
                Op::Mul => lval * rval,
                Op::Div => lval / rval,
                Op::Pow => lval.pow(rval),
            }
        }
        Expr::Negative(expr) => {
            let val = eval(expr)?;
            -val
        }
        Expr::Variable(unknown) => return Err(EvalErr::EncounteredUnknown(unknown.to_owned())),
        Expr::Derivative(expr) => eval(&derive(&expr))?,
        Expr::Func(func, _) => return Err(EvalErr::NonEvaluableFunc(func.to_owned())),
    };

    Ok(answer.simplified())
}
