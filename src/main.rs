use std::path::Path;

use maths_problem_gen::{gen::gen_backtrack, render::render, solve::solve};

fn main() {
    let (equation, answer) = gen_backtrack(2);
    println!("{}, x = {}", &equation, &answer);
    let solved_answer = solve(&equation).expect("Should be solvable");
    assert_eq!(answer, solved_answer);

    render(&equation, &Path::new("out.png")).expect("render to png should succeed");
}
