use itertools::Itertools;
use project_root::get_project_root;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug, Clone, Copy)]
enum PrimToken<N> {
    Open,
    Close,
    Value(N),
    Add,
    Mul,
}

type Token = PrimToken<i64>;

fn parse_tokens(input: &str) -> Vec<Token> {
    use PrimToken::*;
    let mut input = input.trim().chars().peekable();
    let mut output = Vec::new();
    while let Some(c) = input.next() {
        match c {
            '(' => output.push(Open),
            ')' => output.push(Close),
            '+' => output.push(Add),
            '*' => output.push(Mul),
            c if c.is_ascii_digit() => {
                let mut v = c.to_digit(10).unwrap();
                'a: while let Some(c) = input.peek() {
                    match c {
                        c if c.is_ascii_digit() => {
                            v *= 10;
                            v += c.to_digit(10).unwrap();
                            input.next();
                        }
                        _ => {
                            break 'a;
                        }
                    }
                }
                output.push(Value(v as i64));
            }
            _ => {}
        }
    }
    output
}
fn solve_a(input: &[Token]) -> Option<i64> {
    use PrimToken::*;
    let mut input = input.iter();
    let mut current_token = input.next().copied();
    let mut current_thunk = Vec::new();
    let mut thunk = Vec::new();
    while let Some(t) = current_token {
        match t {
            Close => {
                let mut c = std::mem::take(&mut current_thunk);
                let mut c = c.drain(..);
                let mut x: i64;
                if let Some(Value(n)) = c.next() {
                    x = n;
                    while let (Some(op), Some(Value(y))) = (c.next(), c.next()) {
                        match op {
                            Mul => {
                                x *= y;
                            }
                            Add => {
                                x += y;
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                } else {
                    return None;
                }
                if let Some(c) = thunk.pop() {
                    current_thunk = c;
                }
                current_token = Some(Value(x));
            }
            Open => {
                let c = std::mem::take(&mut current_thunk);
                thunk.push(c);
                current_token = input.next().copied();
            }
            Value(v) => {
                match (current_thunk.pop(), current_thunk.pop()) {
                    (None, _) => {
                        current_thunk.push(Value(v));
                    }
                    (Some(Add), Some(Value(x))) => {
                        current_thunk.push(Value(v + x));
                    }
                    (Some(Mul), Some(Value(x))) => {
                        current_thunk.push(Value(v * x));
                    }
                    (Some(a), Some(b)) => {
                        current_thunk.push(b);
                        current_thunk.push(a);
                        current_thunk.push(t);
                    }
                    _ => {
                        return None;
                    }
                }
                current_token = input.next().copied();
            }
            Add | Mul => {
                current_thunk.push(t);
                current_token = input.next().copied();
            }
        }
    }
    let mut c = std::mem::take(&mut current_thunk);
    let mut c = c.drain(..);
    let mut x: i64;
    if let Some(Value(n)) = c.next() {
        x = n;
        while let (Some(op), Some(Value(y))) = (c.next(), c.next()) {
            match op {
                Mul => {
                    x *= y;
                }
                Add => {
                    x += y;
                }
                _ => {
                    return None;
                }
            }
        }
    } else {
        return None;
    }
    Some(x)
}
fn solve_b(input: &[Token]) -> Option<i64> {
    use PrimToken::*;
    let mut input = input.iter();
    let mut current_token = input.next().copied();
    let mut current_thunk = Vec::new();
    let mut thunk = Vec::new();
    while let Some(t) = current_token {
        match t {
            Close => {
                let mut c = std::mem::take(&mut current_thunk);
                let mut c = c.drain(..);
                let mut x: i64;
                if let Some(Value(n)) = c.next() {
                    x = n;
                    while let (Some(op), Some(Value(y))) = (c.next(), c.next()) {
                        match op {
                            Mul => {
                                x *= y;
                            }
                            Add => {
                                x += y;
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                } else {
                    return None;
                }
                if let Some(c) = thunk.pop() {
                    current_thunk = c;
                }
                current_token = Some(Value(x));
            }
            Open => {
                let c = std::mem::take(&mut current_thunk);
                thunk.push(c);
                current_token = input.next().copied();
            }
            Value(v) => {
                match (current_thunk.pop(), current_thunk.pop()) {
                    (None, _) => {
                        current_thunk.push(Value(v));
                    }
                    (Some(Add), Some(Value(x))) => {
                        current_thunk.push(Value(v + x));
                    }
                    (Some(a), Some(b)) => {
                        current_thunk.push(b);
                        current_thunk.push(a);
                        current_thunk.push(t);
                    }
                    _ => {
                        return None;
                    }
                }
                current_token = input.next().copied();
            }
            Add | Mul => {
                current_thunk.push(t);
                current_token = input.next().copied();
            }
        }
    }
    let mut c = std::mem::take(&mut current_thunk);
    let mut c = c.drain(..);
    let mut x: i64;
    if let Some(Value(n)) = c.next() {
        x = n;
        while let (Some(op), Some(Value(y))) = (c.next(), c.next()) {
            match op {
                Mul => {
                    x *= y;
                }
                Add => {
                    x += y;
                }
                _ => {
                    return None;
                }
            }
        }
    } else {
        return None;
    }
    Some(x)
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.lines().map(parse_tokens).collect_vec();
    println!("day18a: {}", input.iter().filter_map(|x| solve_a(x)).sum::<i64>());
    println!("day18b: {}", input.iter().filter_map(|x| solve_b(x)).sum::<i64>());
}
