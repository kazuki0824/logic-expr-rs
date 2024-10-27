use nom::{
    branch::alt,
    character::complete::{char, alpha1, multispace0},
    combinator::map,
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

// 定義するASTノード
#[derive(Debug)]
pub enum Expr {
    Var(String),   // 変数
    Not(Box<Expr>), // 否定
    And(Box<Expr>, Box<Expr>), // 論理積
    Or(Box<Expr>, Box<Expr>),  // 論理和
}

// トークンを解析し、構文木に変換する関数
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    parse_or(input)
}

// 論理和のパーサー
fn parse_or(input: &str) -> IResult<&str, Expr> {
    let (input, first) = parse_and(input)?;
    let (input, rest) = many0(preceded(tuple((multispace0, char('|'), multispace0)), parse_and))(input)?;

    // 複数の `|` を解析する
    let expr = rest.into_iter().fold(first, |acc, next| Expr::Or(Box::new(acc), Box::new(next)));
    Ok((input, expr))
}

// 論理積のパーサー
fn parse_and(input: &str) -> IResult<&str, Expr> {
    let (input, first) = parse_not(input)?;
    let (input, rest) = many0(preceded(tuple((multispace0, char('&'), multispace0)), parse_not))(input)?;

    // 複数の `&` を解析する
    let expr = rest.into_iter().fold(first, |acc, next| Expr::And(Box::new(acc), Box::new(next)));
    Ok((input, expr))
}

// 否定のパーサー
fn parse_not(input: &str) -> IResult<&str, Expr> {
    alt((
        map(preceded(tuple((multispace0, char('!'), multispace0)), parse_atom), |e| Expr::Not(Box::new(e))),
        parse_atom
    ))(input)
}

// 変数や括弧のパーサー
fn parse_atom(input: &str) -> IResult<&str, Expr> {
    alt((
        map(alpha1, |s: &str| Expr::Var(s.to_string())),
        delimited(char('('), parse_expr, char(')')),
        delimited(char('{'), parse_expr, char('}')),
        delimited(char('['), parse_expr, char(']'))
    ))(input)
}
