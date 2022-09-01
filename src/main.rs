use maths_problem_gen::{gen::gen_polynomial, simplify::simplify};
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

    let lhs = gen_polynomial(3);
    println!("{:?}", lhs);

    let ddx = simplify(&derive(&lhs));

    render_to_file(&lhs, &Path::new("lhs.png"), None, false)
        .await
        .expect("render to png should succeed");

    render_to_file(&ddx, &Path::new("ddx.png"), None, false)
        .await
        .expect("render to png should succeed");
}
