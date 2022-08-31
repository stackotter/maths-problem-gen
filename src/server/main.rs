#[macro_use]
extern crate rocket;

use maths_problem_gen::{gen::gen_choices, Answer, Equation, Expr};
use rand::Rng;
use rocket::serde::{json::Json, Serialize};
use std::error::Error;

use maths_problem_gen::{gen::gen_backtrack, render::render_to_bytes};

async fn generate_problem() -> Result<Problem, Box<dyn Error>> {
    let (equation, answer) = gen_backtrack(2);

    let mut choices = gen_choices(answer, 3);
    let letters = ['a', 'b', 'c', 'd'];
    let answer_index = rand::thread_rng().gen_range(0..(choices.len() + 1));
    choices.insert(answer_index, answer);

    let problem_bytes = render_to_bytes(&equation).await?;
    let mut choice_images = vec![];

    for (i, choice) in choices.iter().enumerate() {
        choice_images.push(base64::encode(
            render_to_bytes(&Answer {
                option: letters[i],
                answer: Equation {
                    lhs: Expr::Variable('x'),
                    rhs: choice.to_owned().into(),
                },
            })
            .await?,
        ));
    }

    Ok(Problem {
        problem_image: base64::encode(problem_bytes),
        answer: answer_index,
        choice_images,
    })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Problem {
    problem_image: String,
    answer: usize,
    choice_images: Vec<String>,
}

#[get("/rand-problem")]
async fn rand_problem() -> Result<Json<Problem>, String> {
    generate_problem()
        .await
        .map(|prob| Json(prob))
        .map_err(|err| format!("{:?}", err))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![rand_problem])
}
