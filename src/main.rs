use std::path::Path;
use maths_problem_gen::{render::render, solve::solve, gen::gen_equation, gen::gen_simple_add};

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

    render(&equation, &Path::new("out.png")).expect("render to pdf should succeed");

    // println!("{}", simplify(&derive(&equation.lhs)));
    // println!(
    //     "{}",
    //     simplify(&derive(&Expr::Pair(Box::new(Pair::new(
    //         Expr::Variable('x'),
    //         Op::Mul,
    //         Expr::Variable('x')
    //     )))))
    // );

    //println!("{}", gen_simple_add(5));
}
