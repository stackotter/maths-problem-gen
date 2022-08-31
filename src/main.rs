use maths_problem_gen::{simplify::simplify, Expr, Op, Pair};

use maths_problem_gen::{derive::derive, gen::gen_equation, solve::solve};

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

    println!("{}", simplify(&derive(&equation.lhs)));
    println!(
        "{}",
        simplify(&derive(&Expr::Pair(Box::new(Pair::new(
            Expr::Variable('x'),
            Op::Mul,
            Expr::Variable('x')
        )))))
    );
}
