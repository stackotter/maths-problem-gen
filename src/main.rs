use maths_problem_gen::simplify::simplify;
use std::path::Path;

use maths_problem_gen::{
    derive::derive, gen::gen_backtrack, render::render_to_file, solve::solve, Expr, Pair, Rational, Op,
};
use tokio;

#[tokio::main]
async fn main() {
    // let (equation, answer) = gen_backtrack(2);
    // println!("{}, x = {}", &equation, &answer);
    // let solved_answer = solve(&equation).expect("Should be solvable");
    // assert_eq!(answer, solved_answer);

    let lhs = Expr::Pair(Box::new(Pair::new(
        Rational::int(5).into(),
        Op::Sub,
        Expr::Pair(Box::new(Pair::new(
            Expr::Variable('x'),
            Op::Pow,
            Rational::int(5).into(),
        ))),
    )));

    let ddx = simplify(&derive(&lhs));

    render_to_file(&lhs, &Path::new("lhs.png"), None, false)
        .await
        .expect("render to png should succeed");

    render_to_file(&ddx, &Path::new("ddx.png"), None, false)
        .await
        .expect("render to png should succeed");
}
