use crate::{fmt::bracketize, Equation, Expr, Op, Pair, Rational, Answer};
use std::{
    error::Error,
    fs::File,
    io::{copy, Cursor},
    path::Path, collections::HashMap,
};

use reqwest::Client;

pub trait LatexConvertible {
    fn to_latex(&self) -> String;
}

impl LatexConvertible for Rational {
    fn to_latex(&self) -> String {
        if self.denominator == 1 {
            format!("{}", self.numerator)
        } else {
            format!("\\frac{{{}}}{{{}}}", self.numerator, self.denominator)
        }
    }
}

impl LatexConvertible for Pair {
    fn to_latex(&self) -> String {
        let (lrequires_brackets, rrequires_brackets) = self.requires_brackets(true);

        let mut l: String = self.left.to_latex();
        let mut r: String = self.right.to_latex();

        if lrequires_brackets {
            l = bracketize(&l);
        }
        if rrequires_brackets {
            r = bracketize(&r);
        }

        match self.op {
            Op::Add | Op::Sub => format!("{} {} {}", l, self.op, r),
            Op::Mul => format!("{} \\times {}", l, r),
            Op::Div => format!("\\frac{{{}}}{{{}}}", l, r),
        }
    }
}

impl LatexConvertible for Expr {
    fn to_latex(&self) -> String {
        match self {
            Expr::Rational(rational) => rational.to_latex(),
            Expr::Pair(pair) => pair.to_latex(),
            Expr::Negative(expr) => {
                format!("-{}", expr.to_latex())
            }
            Expr::Variable(var) => var.to_owned().into(),
        }
    }
}

impl LatexConvertible for Equation {
    fn to_latex(&self) -> String {
        format!("{} = {}", self.lhs.to_latex(), self.rhs.to_latex())
    }
}

impl<T: LatexConvertible> LatexConvertible for Answer<T> {
    fn to_latex(&self) -> String {
        format!("{}) \\ {}", self.option, self.answer.to_latex())
    }
}

pub async fn render_to_bytes<T: LatexConvertible>(maths: &T) -> Result<Vec<u8>, Box<dyn Error>> {
    let latex = maths.to_latex();

    let mut params = HashMap::new();
    params.insert("q", latex);
    let client = Client::new();
    let response = client
        .post("http://localhost:10044/png")
        .form(&params)
        .send().await?;

    Ok(response.bytes().await?.to_vec())
}

pub async fn render_to_file<T: LatexConvertible>(maths: &T, file: &Path) -> Result<(), Box<dyn Error>> {
    let bytes = render_to_bytes(maths).await?;
    let mut file = File::create(file)?;
    let mut content = Cursor::new(bytes);
    copy(&mut content, &mut file)?;
    Ok(())
}

