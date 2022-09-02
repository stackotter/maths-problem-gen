#[macro_use]
extern crate rocket;

use maths_problem_gen::gen::{gen_backtrack, gen_polynomial, gen_polynomial_choices};
use maths_problem_gen::render::LatexConvertible;
use maths_problem_gen::simplify::simplify;
use maths_problem_gen::{gen::gen_choices, render::render_to_file, Answer, Equation, Expr};
use rand::Rng;
use rocket::fs::NamedFile;
use rocket::{
    serde::{json::Json, Serialize},
    State,
};
use std::env;
use std::path::PathBuf;
use std::{error::Error, fs, path::Path};

type Maths = Box<dyn LatexConvertible + Send + Sync>;
type Choice = Answer<Maths>;

fn generate_multiple_choice_problem(level: u64) -> Result<(Maths, Vec<Choice>, usize), String> {
    let (problem, answer, mut choices): (Maths, Maths, Vec<Maths>) = match level {
        1 => {
            let (equation, answer) = gen_backtrack(2);

            let choices: Vec<Maths> = gen_choices(answer, 3)
                .into_iter()
                .map(|x| Equation {
                    lhs: Expr::Variable('x'),
                    rhs: x.into(),
                })
                .map(|c| -> Maths { Box::new(c) })
                .collect();

            let answer = Equation {
                lhs: Expr::Variable('x'),
                rhs: answer.into(),
            };

            (
                Box::new(equation),
                Box::new(answer),
                choices,
            )
        }
        2 => {
            let problem = Expr::Derivative(Box::new(gen_polynomial(4)));
            let answer = simplify(&problem);
            let choices: Vec<Maths> = gen_polynomial_choices(&answer, 3)
                .into_iter()
                .map(|c| -> Maths { Box::new(c) })
                .collect();

            (Box::new(problem), Box::new(answer), choices)
        }
        _ => return Err(format!("Invalid level {level}")),
    };

    let answer_index = rand::thread_rng().gen_range(0..(choices.len() + 1));
    choices.insert(answer_index, answer);

    let letters = ['a', 'b', 'c', 'd'];
    let choices = choices
        .into_iter()
        .enumerate()
        .map(|(i, choice)| Answer {
            option: letters[i],
            answer: choice,
        })
        .collect();

    Ok((problem, choices, answer_index))
}

async fn generate_problem(problems_dir: &str, level: u64) -> Result<Problem, Box<dyn Error>> {
    let env_var = env::var("mathoid_server").ok();
    let mathoid_server: Option<&str> = env_var.as_deref();
    let problem_uuid = uuid::Uuid::new_v4();

    let (problem, choices, answer_index) = generate_multiple_choice_problem(level)?;
    let file = format!("{problem_uuid}_problem.png");
    let path = format!("{}/{}", problems_dir, &file);
    render_to_file(problem.as_ref(), &Path::new(&path), mathoid_server, false).await?;
    let mut choice_urls = vec![];

    for (i, choice) in choices.iter().enumerate() {
        let file = format!("{problem_uuid}_choice{i}.png");
        let path = format!("{}/{}", problems_dir, &file);
        render_to_file(choice, &Path::new(&path), mathoid_server, true).await?;
        choice_urls.push(format!("/problem/{file}"));
    }

    Ok(Problem {
        problem_url: format!("/problem/{file}"),
        answer: answer_index,
        choice_urls,
    })
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Problem {
    problem_url: String,
    answer: usize,
    choice_urls: Vec<String>,
}

#[get("/rand-problem?<level>")]
async fn rand_problem(config: &State<Config>, level: Option<u64>) -> Result<Json<Problem>, String> {
    generate_problem(&config.problems_dir, level.unwrap_or(1))
        .await
        .map(|prob| Json(prob))
        .map_err(|err| format!("{:?}", err))
}

#[get("/problem/<file..>")]
async fn problem(config: &State<Config>, file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(&config.problems_dir).join(file))
        .await
        .ok()
}

struct Config {
    problems_dir: String,
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        problems_dir: "./problems".into(),
    };

    let path = Path::new(&config.problems_dir);
    if !path.exists() {
        fs::create_dir(path).expect("Problems dir creation should work")
    }

    rocket::build()
        .mount("/", routes![rand_problem, problem])
        .manage(config)
}
