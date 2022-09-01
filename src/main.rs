use std::path::Path;

use maths_problem_gen::{gen::gen_backtrack, render::render_to_file, solve::solve};
use tokio;

#[tokio::main]
async fn main() {
    let (equation, answer) = gen_backtrack(2);
    println!("{}, x = {}", &equation, &answer);
    let solved_answer = solve(&equation).expect("Should be solvable");
    assert_eq!(answer, solved_answer);

    render_to_file(&equation, &Path::new("out.png"), false)
        .await
        .expect("render to png should succeed");
}
