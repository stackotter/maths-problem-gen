use crate::{fmt::bracketize, Expr, Op, Pair, Rational};
use std::{
    error::Error,
    fs::File,
    io::{copy, Cursor},
    path::Path,
};

use reqwest::blocking::Client;

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
        }
    }
}

pub fn render(expr: &Expr, file: &Path) -> Result<(), Box<dyn Error>> {
    let latex = expr.to_latex();

    let client = Client::new();
    let response = client
        .get("http://localhost:3000/render")
        .query(&[
            ("input", "latex"),
            ("source", &latex),
            ("output", "png"),
            ("width", "400"),
            ("height", "400"),
        ])
        .send()?;

    let mut file = File::create(file)?;
    let mut content = Cursor::new(response.bytes()?);
    copy(&mut content, &mut file)?;
    Ok(())
}
