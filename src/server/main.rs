#[macro_use]
extern crate rocket;

use std::path::PathBuf;
use maths_problem_gen::gen::gen_backtrack;
use maths_problem_gen::{gen::gen_choices, render::render_to_file, Answer, Equation, Expr};
use rand::Rng;
use rocket::fs::NamedFile;
use rocket::{
    serde::{json::Json, Serialize},
    State,
};
use std::{error::Error, fs, path::Path};

async fn generate_problem(problems_dir: &str) -> Result<Problem, Box<dyn Error>> {
    let problem_uuid = uuid::Uuid::new_v4();
    let (equation, answer) = gen_backtrack(2);

    let mut choices = gen_choices(answer, 3);
    let letters = ['a', 'b', 'c', 'd'];
    let answer_index = rand::thread_rng().gen_range(0..(choices.len() + 1));
    choices.insert(answer_index, answer);

    let file = format!("{problem_uuid}_problem.png");
    let path = format!("{}/{}", problems_dir, &file);
    render_to_file(&equation, &Path::new(&path), false).await?;
    let mut choice_urls = vec![];

    for (i, choice) in choices.iter().enumerate() {
        let file = format!("{problem_uuid}_choice{i}.png");
        let path = format!("{}/{}", problems_dir, &file);
        render_to_file(
            &Answer {
                option: letters[i],
                answer: Equation {
                    lhs: Expr::Variable('x'),
                    rhs: choice.to_owned().into(),
                },
            },
            &Path::new(&path),
            true
        )
        .await?;
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

#[get("/rand-problem")]
async fn rand_problem(config: &State<Config>) -> Result<Json<Problem>, String> {
    generate_problem(&config.problems_dir)
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
