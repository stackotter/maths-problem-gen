use std::path::Path;

use maths_problem_gen::{eval::eval, gen::gen_all, render, Equation};

fn main() {
    let expr = gen_all();
    let ans = eval(&expr);
    let equation = Equation {
        lhs: expr.clone(),
        rhs: ans.clone().into(),
    };
    println!("{}", &equation);
    render::render(&equation, &Path::new("out.png")).expect("render to pdf should succeed");
}
