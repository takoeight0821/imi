use std::collections::HashMap;

/// Definitions of S-expressions
#[derive(Debug, PartialEq)]
enum Expr {
    Literal(i32),
    Symbol(String),
    List(Vec<Expr>),
}

#[derive(Debug)]
enum Error {
    Reason(String),
}

struct Env {
    parent: Option<Box<Env>>,
    bindings: HashMap<String, Expr>,
}

/// tokenize a string
/// e.g. "(1 2 3)" -> ["(", "1", "2", "3", ")"]
fn tokenize(s: &str) -> Vec<String> {
    s.replace('(', " ( ")
        .replace(')', " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

/// parse a tokenized string
fn parse(tokens: &[String]) -> Result<(Expr, &[String]), Error> {
    let (token, rest) = tokens.split_first().ok_or_else(|| Error::Reason("empty input".to_string()))?;
    match &token[..] {
        "(" => read_list(rest),
        ")" => Err(Error::Reason("unexpected )".to_string())),
        _ => Ok((read_atom(token), rest)),
    }
}

/// read a list beginning with "(" and ending with ")"
/// e.g. ["(", "1", "2", "3", ")"] -> List(vec![Literal(1), Literal(2), Literal(3)]) 
///      ["(", ")"] -> List(vec![])
fn read_list(tokens: &[String]) -> Result<(Expr, &[String]), Error> {
    let mut tokens = tokens;
    let mut list = Vec::new();
    while let Some((next, rest)) = tokens.split_first() {
        if next == ")" {
            return Ok((Expr::List(list), rest));
        }
        let (expr, rest) = parse(tokens)?;
        list.push(expr);
        tokens = rest;    
    }
    Err(Error::Reason("could not parse list".to_string()))
}

/// read an atom
fn read_atom(token: &str) -> Expr {
    match token.parse::<i32>() {
        Ok(n) => Expr::Literal(n),
        Err(_) => Expr::Symbol(token.to_string()),
    }
}


fn main() {
    println!("{:?}", tokenize("(+ 1 2)"));
    println!("{:?}", parse(&tokenize("(+ 1 2)")));
}
