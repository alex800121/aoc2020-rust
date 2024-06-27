use aoc2020::Direction::{self, *};
use project_root::get_project_root;

type Coord = [isize; 2];

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Copy)]
struct Ferry {
    coord: Coord,
    waypoint: Coord,
}
#[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Clone, Copy)]
enum Ins {
    D(Direction, isize),
    F(isize),
    T(usize),
}
impl Ferry {
    fn coord_dir(&mut self, n: isize, d: Direction) {
        let [x, y] = d.to_index();
        self.coord = [self.coord[0] + (x * n), self.coord[1] + (y * n)];
    }
    fn way_dir(&mut self, n: isize, d: Direction) {
        let [x, y] = d.to_index();
        self.waypoint = [self.waypoint[0] + (x * n), self.waypoint[1] + (y * n)];
    }
    fn forward(&mut self, n: isize) {
        let [x, y] = self.waypoint;
        self.coord = [self.coord[0] + (x * n), self.coord[1] + (y * n)];
    }
    fn way_turn(&mut self, n: usize) {
        for _ in 0..n {
            self.waypoint = [-self.waypoint[1], self.waypoint[0]];
        }
    }
}
fn parse_ins(input: &str) -> Option<Ins> {
    use Ins::*;
    let (x, y) = input.split_at(1);
    match (x, y.parse::<isize>().ok()?) {
        ("N", n) => Some(D(North, n)),
        ("S", n) => Some(D(South, n)),
        ("E", n) => Some(D(East, n)),
        ("W", n) => Some(D(West, n)),
        ("F", n) => Some(F(n)),
        ("R", n) => Some(T((n / 90) as usize % 4)),
        ("L", n) => Some(T((-n / 90).rem_euclid(4) as usize)),
        _ => None,
    }
}
fn step_b(input: &str, ferry: &mut Ferry) {
    use Ins::*;
    match parse_ins(input) {
        Some(D(d, n)) => ferry.way_dir(n, d),
        Some(F(n)) => ferry.forward(n),
        Some(T(n)) => ferry.way_turn(n),
        _ => {}
    }
}
fn step_a(input: &str, ferry: &mut Ferry) {
    use Ins::*;
    match parse_ins(input) {
        Some(D(d, n)) => ferry.coord_dir(n, d),
        Some(F(n)) => ferry.forward(n),
        Some(T(n)) => ferry.way_turn(n),
        _ => {}
    }
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        // "{}/input/test{:02}.txt",
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input_a = input.lines();
    let mut ferry_a = Ferry {
        coord: [0, 0],
        waypoint: (East.to_index()),
    };
    let mut ferry_b = Ferry {
        coord: [0, 0],
        waypoint: [10, -1],
    };
    for input in input_a {
        step_a(input, &mut ferry_a);
        step_b(input, &mut ferry_b);
        // dbg!(&ferry_a);
    }
    println!("day12a: {}", ferry_a.coord[0].abs() + ferry_a.coord[1].abs());
    println!("day12b: {}", ferry_b.coord[0].abs() + ferry_b.coord[1].abs());
}
