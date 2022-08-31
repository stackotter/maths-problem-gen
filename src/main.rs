use std::path::Path;

use maths_problem_gen::{eval::eval, gen::{gen_arithmetic, gen_backtrack}, render::render, Equation, solve::solve};

fn main() {
    let (equation, answer) = gen_backtrack(2);
    println!("{}, x = {}", &equation, &answer);
    let solved_answer = solve(&equation).expect("Should be solvable");
    assert_eq!(answer, solved_answer);

    render(&equation, &Path::new("out.png")).expect("render to pdf should succeed");
}
