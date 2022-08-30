use std::path::Path;

use maths_problem_gen::{eval::eval, gen::{gen_all, gen_equation}, render, Equation, solve::solve, derive::derive};

fn main() {
    // let expr = gen_all();
    // let ans = eval(&expr);
    // let equation = Equation {
    //     lhs: expr.clone(),
    //     rhs: ans.clone().into(),
    // };
    // println!("{}", &equation);

    let equation = gen_equation();
    let answer = solve(&equation).expect("Should be solvable");
    println!("{}, x = {}", equation, answer);

    // render::render(&equation, &Path::new("out.png")).expect("render to pdf should succeed");

    println!("{}", derive(&equation.lhs))
}
