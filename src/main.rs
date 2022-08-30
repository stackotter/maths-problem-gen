use maths_problem_gen::{eval::eval, gen::gen_all};

fn main() {
    let expr = gen_all();
    let ans = eval(&expr);
    println!("{} = {}", expr, ans);
}
