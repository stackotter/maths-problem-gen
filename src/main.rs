use maths_problem_gen::{
    gen::{gen_derivable, gen_polynomial},
    simplify::simplify,
    Equation, Func,
};
use std::path::Path;

use maths_problem_gen::{
    derive::derive, gen::gen_backtrack, render::render_to_file, solve::solve, Expr, Op, Pair,
    Rational,
};
use tokio;

#[tokio::main]
async fn main() {
    // let (equation, answer) = gen_backtrack(2);
    // println!("{}, x = {}", &equation, &answer);
    // let solved_answer = solve(&equation).expect("Should be solvable");
    // assert_eq!(answer, solved_answer);

    // let expr = Pair::new(
    //     Pair::new(
    //         Pair::new(Expr::Variable('x'), Op::Pow, Rational::int(3).into()).into(),
    //         Op::Add,
    //         Pair::new(Expr::Variable('x'), Op::Pow, Rational::int(2).into()).into(),
    //     ).into(),
    //     Op::Pow,
    //     Rational::int(2).into(),
    // ).into();

    let expr = Pair::new(
        Expr::Variable('x'),
        Op::Div,
        Pair::new(Expr::Variable('x'), Op::Pow, Rational::int(3).into()).into(),
    )
    .into();

    render_to_file(
        &simplify(&derive(&expr)),
        &Path::new("out.png"),
        None,
        false,
    )
    .await
    .expect("render to png should succeed");
}
