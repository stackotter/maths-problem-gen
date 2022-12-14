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
                    (Expr::Rational(rational), expr) | (expr, Expr::Rational(rational))
                        if rational == Rational::int(1) =>
                    {
                        return expr;
                    }
                    (left, right) if left == right => {
                        return Pair::new(left, Op::Pow, Rational::int(2).into()).into();
                    }
                    (
                        left,
                        Expr::Pair(box Pair {
                            left: Expr::Rational(rational),
                            op: Op::Mul,
                            right,
                        }),
                    ) => {
                        return Pair::new(
                            rational.into(),
                            Op::Mul,
                            simplify(&Pair::new(left, Op::Mul, right).into()),
                        )
                        .into();
                    }
                    (
                        left,
                        Expr::Pair(box Pair {
                            left: right,
                            op: Op::Pow,
                            right: exponent,
                        }),
                    )
                    | (
                        Expr::Pair(box Pair {
                            left,
                            op: Op::Pow,
                            right: exponent,
                        }),
                        right,
                    ) if left == right => {
                        return Pair::new(
                            left,
                            Op::Pow,
                            simplify(&Pair::new(exponent, Op::Add, Rational::int(1).into()).into()),
                        )
                        .into();
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
                            right: lexpr,
                        }),
                        Expr::Pair(box Pair {
                            left: Expr::Rational(rcoefficient),
                            op: Op::Mul,
                            right: rexpr,
                        }),
                    ) if lexpr == rexpr => {
                        let new_coefficient = lcoefficient + rcoefficient;
                        return Expr::Pair(Box::new(Pair::new(
                            new_coefficient.into(),
                            Op::Mul,
                            lexpr,
                        )));
                    }
                    _ => (),
                },
                Op::Sub => match (lsimplified.clone(), rsimplified.clone()) {
                    (Expr::Rational(rational), right) if rational.numerator == 0 => {
                        return Expr::Negative(Box::new(right))
                    }
                    (left, Expr::Rational(rational)) if rational.numerator == 0 => return left,
                    (
                        Expr::Pair(box Pair {
                            left: Expr::Rational(lcoefficient),
                            op: Op::Mul,
                            right: lexpr,
                        }),
                        Expr::Pair(box Pair {
                            left: Expr::Rational(rcoefficient),
                            op: Op::Mul,
                            right: rexpr,
                        }),
                    ) if lexpr == rexpr => {
                        let new_coefficient = lcoefficient - rcoefficient;
                        return Pair::new(
                            new_coefficient.into(),
                            Op::Mul,
                            lexpr,
                        ).into();
                    }
                    (
                        lexpr,
                        Expr::Pair(box Pair {
                            left: Expr::Rational(rcoefficient),
                            op: Op::Mul,
                            right: rexpr,
                        }),
                    ) if lexpr == rexpr => {
                        let new_coefficient = Rational::int(1) - rcoefficient;
                        return Pair::new(
                            new_coefficient.into(),
                            Op::Mul,
                            lexpr,
                        ).into();
                    }
                    (
                        Expr::Pair(box Pair {
                            left: Expr::Rational(lcoefficient),
                            op: Op::Mul,
                            right: lexpr,
                        }),
                        rexpr,
                    ) if lexpr == rexpr => {
                        let new_coefficient = lcoefficient - Rational::int(1);
                        return Pair::new(
                            new_coefficient.into(),
                            Op::Mul,
                            lexpr,
                        ).into();
                    }
                    _ => (),
                },
                Op::Pow if rsimplified == Rational::int(0).into() => {
                    return Rational::int(1).into();
                }
                Op::Pow if rsimplified == Rational::int(1).into() => return lsimplified.clone(),
                Op::Pow => match (lsimplified.clone(), rsimplified.clone()) {
                    (
                        Expr::Pair(box Pair {
                            left: base,
                            op: Op::Pow,
                            right: inner_power,
                        }),
                        outer_power,
                    ) => {
                        return Pair::new(
                            base,
                            Op::Pow,
                            simplify(&Pair::new(inner_power, Op::Mul, outer_power).into()),
                        )
                        .into();
                    }
                    _ => (),
                },
                Op::Div => match (lsimplified.clone(), rsimplified.clone()) {
                    (
                        Expr::Pair(box Pair {
                            left: numerator,
                            op: Op::Div,
                            right: denominator,
                        }),
                        second_denominator,
                    ) => {
                        return Pair::new(
                            numerator,
                            Op::Div,
                            Pair::new(denominator, Op::Mul, second_denominator).into(),
                        )
                        .into()
                    }
                    (
                        numerator,
                        Expr::Pair(box Pair {
                            left: denominator,
                            op: Op::Div,
                            right: second_numerator,
                        }),
                    ) => {
                        return Pair::new(
                            Pair::new(numerator, Op::Mul, second_numerator).into(),
                            Op::Div,
                            denominator,
                        )
                        .into()
                    }
                    _ => (),
                },
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
                Expr::Pair(_)
                | Expr::Rational(_)
                | Expr::Variable(_)
                | Expr::Derivative(_)
                | Expr::Func(_, _) => Expr::Negative(Box::new(simplified)),
            }
        }
        Expr::Variable(_) => expr.to_owned(),
        Expr::Derivative(inner) => simplify(&derive(inner)),
        Expr::Func(func, inner) => Expr::Func(*func, Box::new(simplify(&inner))),
    }
}
