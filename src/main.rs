use std::path::Path;

use maths_problem_gen::{eval::eval, gen::gen_all, render};

fn main() {
    let expr = gen_all();
    let ans = eval(&expr);
    println!("{} = {}", expr, ans);
    render::render(&expr, &Path::new("out.png")).expect("render to pdf should succeed");
}
