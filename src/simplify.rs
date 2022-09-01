use crate::{derive::derive, eval::eval, Expr, Op, Pair, Rational};

pub fn simplify(expr: &Expr) -> Expr {
    if let Ok(answer) = eval(expr) {
        return answer.into();
    }

    match expr {
        Expr::Rational(_) => expr.to_owned(),
        Expr::Pair(pair) => {
            let lsimplified = simplify(&pair.left);
            let rsimplified = simplify(&pair.right);

            if let Ok(answer) =
                eval(&Pair::new(lsimplified.clone(), pair.op, rsimplified.clone()).into())
            {
                return answer.into();
            }

            match pair.op {
                Op::Mul => match (lsimplified.clone(), rsimplified.clone()) {
                    (Expr::Rational(rational), _) | (_, Expr::Rational(rational))
                        if rational.numerator == 0 =>
                    {
                        return Rational::int(0).into()
                    }
                    (
                        Expr::Rational(rational),
                        Expr::Pair(box Pair {
                            left: Expr::Rational(coefficient),
                            op: Op::Mul,
                            right: unknown,
                        }),
                    ) => {
                        let new_coefficient = coefficient * rational;
                        return Expr::Pair(Box::new(Pair::new(
                            new_coefficient.into(),
                            Op::Mul,
                            unknown,
                        )));
                    }
                    (Expr::Variable(_), Expr::Rational(_)) => {
                        return Expr::Pair(Box::new(Pair::new(rsimplified, Op::Mul, lsimplified)));
                    }
                    (Expr::Negative(linner), Expr::Negative(rinner)) => {
                        return Expr::Pair(Box::new(Pair::new(*linner, Op::Mul, *rinner)));
                    }
                    (Expr::Negative(box left), right) | (left, Expr::Negative(box right)) => {
                        return Expr::Negative(Box::new(Pair::new(left, Op::Mul, right).into()));
                    }
                    (Expr::Rational(rational), right) if rational.numerator < 0 => {
                        return Expr::Negative(Box::new(
                            Pair::new((-rational).into(), Op::Mul, right).into(),
                        ));
                    }
                    (Expr::Rational(rational), right) if rational == Rational::int(1) => {
                        return right;
                    }
                    _ => (),
                },
                Op::Add => match (lsimplified.clone(), rsimplified.clone()) {
                    (Expr::Rational(rational), right) if rational.numerator == 0 => return right,
                    (left, Expr::Rational(rational)) if rational.numerator == 0 => return left,
                    (left, Expr::Rational(rational)) if rational.numerator < 0 => {
                        return Pair::new(left, Op::Sub, (-rational).into()).into()
                    }
                    (left, Expr::Negative(right)) => {
                        return Expr::Pair(Box::new(Pair::new(left, Op::Sub, *right)))
                    }
                    (
                        Expr::Pair(box Pair {
                            left: Expr::Rational(lcoefficient),
                            op: Op::Mul,
                            right: Expr::Variable(lunknown),
                        }),
                        Expr::Pair(box Pair {
                            left: Expr::Rational(rcoefficient),
                            op: Op::Mul,
                            right: Expr::Variable(runknown),
                        }),
                    ) if lunknown == runknown => {
                        let new_coefficient = lcoefficient + rcoefficient;
                        return Expr::Pair(Box::new(Pair::new(
                            new_coefficient.into(),
                            Op::Mul,
                            Expr::Variable(lunknown),
                        )));
                    }
                    _ => (),
                },
                Op::Sub => match (lsimplified.clone(), rsimplified.clone()) {
                    (Expr::Rational(rational), right) if rational.numerator == 0 => {
                        return Expr::Negative(Box::new(right))
                    }
                    (left, Expr::Rational(rational)) if rational.numerator == 0 => return left,
                    _ => (),
                },
                Op::Pow if rsimplified == Rational::int(0).into() => {
                    return Rational::int(1).into();
                }
                Op::Pow if rsimplified == Rational::int(1).into() => return lsimplified.clone(),
                _ => (),
            }

            return Expr::Pair(Box::new(Pair::new(
                lsimplified,
                pair.op.clone(),
                rsimplified,
            )));
        }
        Expr::Negative(inner) => {
            let simplified = simplify(inner);
            match simplified {
                Expr::Negative(expr) => *expr,
                Expr::Pair(_) | Expr::Rational(_) | Expr::Variable(_) | Expr::Derivative(_) => {
                    Expr::Negative(Box::new(simplified))
                }
            }
        }
        Expr::Variable(_) => expr.to_owned(),
        Expr::Derivative(_) => simplify(&derive(expr)),
    }
}
