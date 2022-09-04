use crate::{Answer, Equation, Expr, Op, Pair, Rational};
use image::{io::Reader as ImageReader, ImageFormat};
use reqwest::Client;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{copy, Cursor},
    path::Path,
};

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

fn latex_bracketize(s: &str) -> String {
    format!("\\left({s}\\right)")
}

impl LatexConvertible for Pair {
    fn to_latex(&self) -> String {
        let (lrequires_brackets, rrequires_brackets) = self.requires_brackets(true, false);

        let mut l: String = self.left.to_latex();
        let mut r: String = self.right.to_latex();

        if lrequires_brackets {
            l = latex_bracketize(&l);
        }
        if rrequires_brackets {
            r = latex_bracketize(&r);
        }

        match self.op {
            Op::Add | Op::Sub => format!("{} {} {}", l, self.op, r),
            Op::Mul => {
                if rrequires_brackets || lrequires_brackets {
                    return format!("{l} {r}");
                }
                match self.right {
                    Expr::Variable(_)
                    | Expr::Pair(box Pair {
                        left: Expr::Variable(_),
                        op: Op::Pow,
                        ..
                    }) => {
                        format!("{l} {r}")
                    }
                    _ => format!("{l} \\times {r}"),
                }
            }
            Op::Div => format!("\\frac{{{l}}}{{{r}}}"),
            Op::Pow => format!("{l}^{{{r}}}"),
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
            Expr::Derivative(expr) => {
                format!("\\frac{{d}}{{dx}}\\left({}\\right)", expr.to_latex())
            }
            Expr::Func(func, inner) => format!("{}\\left({}\\right)", func, inner.to_latex()),
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

impl LatexConvertible for Box<dyn LatexConvertible + Send + Sync> {
    fn to_latex(&self) -> String {
        LatexConvertible::to_latex(self.as_ref())
    }
}

fn u8_chan_to_f32(val: u8) -> f32 {
    val as f32 / (u8::MAX as f32)
}

fn f32_chan_to_u8(val: f32) -> u8 {
    (val * (u8::MAX as f32)) as u8
}

fn composite(val: u8, alpha: u8, bg: u8) -> u8 {
    let val = u8_chan_to_f32(val);
    let alpha = u8_chan_to_f32(alpha);
    let bg = u8_chan_to_f32(bg);
    f32_chan_to_u8(val * alpha + bg * (1.0 - alpha))
}

pub async fn render_to_bytes<T: LatexConvertible + ?Sized>(
    maths: &T,
    mathoid_server: Option<&str>,
    inline: bool,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mathoid_server = mathoid_server.unwrap_or("http://localhost:10044".into());
    let latex = maths.to_latex();

    let mut params = HashMap::new();
    params.insert("q", latex);
    if inline {
        params.insert("type", "inline-tex".into());
    }

    let client = Client::new();
    let response = client
        .post(&format!("{mathoid_server}/png"))
        .form(&params)
        .send()
        .await?;

    let bytes = response.bytes().await?;

    // make image bg white
    let mut reader = ImageReader::new(Cursor::new(bytes.clone()));
    reader.set_format(image::ImageFormat::Png);
    let mut img = reader.decode()?.into_rgba8();
    for p in img.pixels_mut() {
        let a = p[3];
        if a != u8::MAX {
            for i in 0..3 {
                p[i] = composite(p[i], a, u8::MAX);
            }
            p[3] = u8::MAX;
        }
    }
    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageFormat::Png)?;

    Ok(buf.into_inner())
}

pub async fn render_to_file<T: LatexConvertible + ?Sized>(
    maths: &T,
    file: &Path,
    mathoid_server: Option<&str>,
    inline: bool,
) -> Result<(), Box<dyn Error>> {
    let bytes = render_to_bytes(maths, mathoid_server, inline).await?;
    let mut file = File::create(file)?;
    let mut content = Cursor::new(bytes);
    copy(&mut content, &mut file)?;
    Ok(())
}
