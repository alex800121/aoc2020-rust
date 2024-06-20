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

type Token = PrimToken<i32>;

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
                            output.push(Value(v as i32));
                            break 'a;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    output
}
fn solve_b(input: &[Token]) -> Option<i32> {
    use PrimToken::*;
    let mut thunk = Vec::new();
    let mut current_thunk = Vec::new();
    let mut input = input.iter().peekable();
    let mut current_token = input.next();
    while let Some(t) = current_token {
        match t {
            Open => todo!(),
            Close => todo!(),
            Value(v) => {

            },
            Add => todo!(),
            Mul => todo!(),
        }
    }
    unimplemented!()
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.lines().map(parse_tokens).collect_vec();
    dbg!(input);
}
