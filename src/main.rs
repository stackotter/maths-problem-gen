use maths_problem_gen::{gen::gen_polynomial, simplify::simplify, Equation, Func};
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

    let inner = gen_polynomial(2);
    let lhs = Pair::new(Expr::Func(Func::Sine, Box::new(inner)), Op::Div, gen_polynomial(2)).into();
    println!("{:?}", lhs);

    let ddx = simplify(&derive(&derive(&lhs)));

    let equation = Equation {
        lhs: Expr::Derivative(Box::new(lhs)),
        rhs: ddx
    };

    render_to_file(&equation, &Path::new("out.png"), None, false)
        .await
        .expect("render to png should succeed");
}
