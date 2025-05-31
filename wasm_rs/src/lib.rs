use nom::{
    IResult,
    character::complete::{char, digit0, multispace0},
    error::Error,
};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
enum ExprAddSub {
    MulDiv(ExprMulDiv),
    Add(Box<ExprAddSub>, ExprMulDiv),
    Sub(Box<ExprAddSub>, ExprMulDiv),
}

impl ExprAddSub {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, _) = multispace0(s)?;
        let (s, muldiv) = ExprMulDiv::parse(s)?;
        let (s, _) = multispace0(s)?;
        if let Ok((s, _)) = char::<&str, Error<&str>>('+')(s) {
            let (s, _) = multispace0(s)?;
            let (s, exp) = Self::parse(s)?;
            Ok((s, Self::Add(Box::new(exp), muldiv)))
        } else if let Ok((s, _)) = char::<&str, Error<&str>>('-')(s) {
            let (s, _) = multispace0(s)?;
            let (s, exp) = Self::parse(s)?;
            Ok((s, Self::Sub(Box::new(exp), muldiv)))
        } else {
            Ok((s, Self::MulDiv(muldiv)))
        }
    }
    fn calc(self) -> i32 {
        match self {
            Self::Add(lhs, rhs) => lhs.calc() + rhs.calc(),
            Self::Sub(lhs, rhs) => lhs.calc() - rhs.calc(),
            Self::MulDiv(muldiv) => muldiv.calc(),
        }
    }
}

#[derive(Debug)]
enum ExprMulDiv {
    Power(ExprPower),
    Mul(Box<ExprMulDiv>, ExprPower),
    Div(Box<ExprMulDiv>, ExprPower),
}

impl ExprMulDiv {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, _) = multispace0(s)?;
        let (s, pow) = ExprPower::parse(s)?;
        let (s, _) = multispace0(s)?;
        if let Ok((s, _)) = char::<&str, Error<&str>>('*')(s) {
            let (s, _) = multispace0(s)?;
            let (s, exp) = Self::parse(s)?;
            Ok((s, Self::Mul(Box::new(exp), pow)))
        } else if let Ok((s, _)) = char::<&str, Error<&str>>('/')(s) {
            let (s, _) = multispace0(s)?;
            let (s, exp) = Self::parse(s)?;
            Ok((s, Self::Div(Box::new(exp), pow)))
        } else {
            Ok((s, Self::Power(pow)))
        }
    }
    fn calc(self) -> i32 {
        match self {
            Self::Power(pow) => pow.calc(),
            Self::Mul(left, right) => left.calc() * right.calc(),
            Self::Div(left, right) => left.calc() / right.calc(),
        }
    }
}

#[derive(Debug)]
enum ExprPower {
    Num(ExprBrackets),
    Exp(Box<ExprPower>, ExprBrackets),
}

impl ExprPower {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, _) = multispace0(s)?;
        let (s, bracket) = ExprBrackets::parse(s)?;
        let (s, _) = multispace0(s)?;
        if let Ok((s, _)) = char::<&str, Error<&str>>('^')(s) {
            let (s, _) = multispace0(s)?;
            let (s, exp) = Self::parse(s)?;
            Ok((s, Self::Exp(Box::new(exp), bracket)))
        } else {
            Ok((s, Self::Num(bracket)))
        }
    }
    fn calc(self) -> i32 {
        match self {
            Self::Num(num) => num.calc(),
            Self::Exp(exp, bracket) => {
                let base = exp.calc();
                let bracket = bracket.calc() as f64;
                bracket.powi(base) as i32
            }
        }
    }
}

#[derive(Debug)]
enum ExprBrackets {
    Num(i32),
    Exp(Box<ExprAddSub>),
}

impl ExprBrackets {
    fn parse(s: &str) -> IResult<&str, Self> {
        let (s, _) = multispace0(s)?;
        let (s, num) = digit0(s)?;
        if let Ok(n) = num.parse::<i32>() {
            Ok((s, Self::Num(n)))
        } else {
            let (s, _) = char('(')(s)?;
            let (s, _) = multispace0(s)?;
            let (s, exp) = ExprAddSub::parse(s)?;
            let (s, _) = multispace0(s)?;
            let (s, _) = char(')')(s)?;
            Ok((s, Self::Exp(Box::new(exp))))
        }
    }
    fn calc(self) -> i32 {
        match self {
            Self::Num(n) => n,
            Self::Exp(exp) => exp.calc(),
        }
    }
}

#[wasm_bindgen]
pub fn math_parse(s: &str) -> Option<i32> {
    let (_, expr) = ExprAddSub::parse(s).ok()?;
    let ans = expr.calc();
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn num_parse() {
        let base = "1";
        let ans = 1;
        let (_, parse) = ExprAddSub::parse(base).unwrap();
        let res = parse.calc();
        assert_eq!(res, ans);
    }
    #[test]
    fn add_parse() {
        let base = "1+ 2";
        let ans = 3;
        let (_, parse) = ExprAddSub::parse(base).unwrap();
        let res = parse.calc();
        assert_eq!(res, ans);
    }
    #[test]
    fn mul_parse() {
        let base = "1* 2";
        let ans = 2;
        let (_, parse) = ExprAddSub::parse(base).unwrap();
        let res = parse.calc();
        assert_eq!(res, ans);
    }
    #[test]
    fn add_mul_parse() {
        let base = "2* 2 + 3";
        let ans = 7;
        let (_, parse) = ExprAddSub::parse(base).unwrap();
        let res = parse.calc();
        assert_eq!(res, ans);
    }
    #[test]
    fn bracket_add_mul_parse() {
        let base = "2* (2 + 3)";
        let ans = 10;
        let (_, parse) = ExprAddSub::parse(base).unwrap();
        let res = parse.calc();
        assert_eq!(res, ans);
    }
    #[test]
    #[allow(non_snake_case)]
    fn POWER() {
        let base = "2 ^ (1 + 3 * 2)";
        let ans = 128;
        let (_, parse) = ExprAddSub::parse(base).unwrap();
        let res = parse.calc();
        assert_eq!(res, ans);
    }
}
