use maths_problem_gen::{eval::eval, gen::gen_addition};

fn main() {
    let expr = gen_addition();
    println!("{} = {}", expr, eval(&expr));
}
