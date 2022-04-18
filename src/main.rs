use std::collections::HashMap;

/// Definitions of S-expressions
#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Number(f64),
    Symbol(String),
    List(Vec<Expr>),
    Func(fn(Vec<Expr>) -> Result<Expr, Error>),
}

#[derive(Debug)]
enum Error {
    Reason(String),
}

struct Env {
    parent: Option<Box<Env>>,
    bindings: HashMap<String, Expr>,
}

fn default_env() -> Env {
    let mut env = Env {
        parent: None,
        bindings: HashMap::new(),
    };
    env.bindings.insert(
        "+".to_string(),
        Expr::Func(|args| {
            let mut sum = 0.0;
            for arg in args {
                match arg {
                    Expr::Number(n) => sum += n,
                    _ => return Err(Error::Reason("+ expects numbers".to_string())),
                }
            }
            Ok(Expr::Number(sum))
        }),
    );
    env.bindings.insert(
        "-".to_string(),
        Expr::Func(|args| {
            let mut sum = match args.get(0) {
                Some(Expr::Number(n)) => *n,
                _ => return Err(Error::Reason("- expects numbers".to_string())),
            };
            for arg in args[1..].iter() {
                match arg {
                    Expr::Number(n) => sum -= n,
                    _ => return Err(Error::Reason("- expects numbers".to_string())),
                }
            }
            Ok(Expr::Number(sum))
        }),
    );
    env
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
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| Error::Reason("empty input".to_string()))?;
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
    match token.parse::<f64>() {
        Ok(float) => Expr::Number(float),
        Err(_) => Expr::Symbol(token.to_string()),
    }
}

fn eval(expr: Expr, env: &Env) -> Result<Expr, Error> {
    match expr {
        Expr::Number(n) => Ok(Expr::Number(n)),
        Expr::Symbol(s) => env
            .bindings
            .get(&s)
            .cloned()
            .ok_or_else(|| Error::Reason(format!("unbound symbol: {}", s))),
        Expr::List(list) => {
            let (func, rest) = list
                .split_first()
                .ok_or_else(|| Error::Reason("empty list".to_string()))?;
            match eval(func.clone(), env) {
                Ok(Expr::Func(f)) => {
                    let args = rest
                        .iter()
                        .cloned()
                        .map(|e| eval(e, env))
                        .collect::<Result<_, _>>()?;
                    f(args)
                }
                Ok(_) => {
                    Err(Error::Reason("not a function".to_string()))
                }
                Err(e) => Err(e),
            }
        }
        Expr::Func(f) => Ok(Expr::Func(f)),
    }
}

fn main() {
    println!("{:?}", tokenize("(+ 1 2)"));
    println!("{:?}", parse(&tokenize("(+ 1 2)")));
    match parse(&tokenize("(+ 1 2)")) {
        Ok((expr, _rest)) => println!("{:?}", eval(expr, &default_env())),
        Err(e) => println!("{:?}", e),
    }
}
