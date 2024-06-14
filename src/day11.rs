use std::array::from_fn;

use itertools::Itertools;
use num::ToPrimitive;
use project_root::get_project_root;

type Index = (usize, usize);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
enum Space {
    Floor,
    Occupied,
    Vacant,
}

const W: usize = 97;
const H: usize = 93;
type Seats = [[Space; W]; H];
const ADJ: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
fn step_a(seats: &Seats) -> Seats {
    from_fn(|y| {
        from_fn(|x| calc_seat(seats, (x, y), 4, |i| adjacent(seats, i)).unwrap_or(Space::Floor))
    })
}
fn step_b(seats: &Seats) -> Seats {
    from_fn(|y| from_fn(|x| calc_seat(seats, (x, y), 5, |i| far(seats, i)).unwrap_or(Space::Floor)))
}
fn get_seat(seats: &Seats, x: isize, y: isize) -> Option<Space> {
    let x = x.to_usize()?;
    let y = y.to_usize()?;
    seats.get(y)?.get(x).copied()
}
fn far(seats: &Seats, i: Index) -> usize {
    let mut output = 0;
    for (x, y) in ADJ {
        let (mut x0, mut y0) = (i.0 as isize + x, i.1 as isize + y);
        while let Some(s) = get_seat(seats, x0, y0) {
            match s {
                Space::Occupied => {
                    output += 1;
                    break;
                }
                Space::Vacant => {
                    break;
                }
                _ => {
                    x0 += x;
                    y0 += y;
                }
            }
        }
    }
    output
}
fn calc_seat(
    seats: &Seats,
    (x, y): Index,
    tolerable: usize,
    adj_f: impl FnOnce(Index) -> usize,
) -> Option<Space> {
    use Space::*;
    let adj = adj_f((x, y));
    let c = seats.get(y)?.get(x)?;
    Some(match c {
        Vacant if adj == 0 => Occupied,
        Occupied if adj >= tolerable => Vacant,
        x => *x,
    })
}
fn solve(seats: &Seats, f: impl Fn(&Seats) -> Seats) -> usize {
    let mut seats = *seats;
    loop {
        let next_seats = f(&seats);
        if seats == next_seats {
            return seats
                .iter()
                .map(|l| l.iter().filter(|x| **x == Space::Occupied).count())
                .sum::<usize>();
        } else {
            seats = next_seats;
        }
    }
}
fn adjacent(m: &Seats, (x, y): Index) -> usize {
    ADJ.iter()
        .filter(|(a, b)| {
            let x = a + x as isize;
            let y = b + y as isize;
            x >= 0
                && y >= 0
                && m.get(y as usize)
                    .is_some_and(|l| l.get(x as usize).is_some_and(|c| c == &Space::Occupied))
        })
        .count()
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let floor: Seats = input
        .lines()
        .filter_map(|l| {
            l.chars()
                .filter_map(|c| match c {
                    '.' => Some(Space::Floor),
                    'L' => Some(Space::Vacant),
                    '#' => Some(Space::Occupied),
                    _ => None,
                })
                .collect_vec()
                .try_into()
                .ok()
        })
        .collect_vec()
        .try_into()
        .unwrap();
    println!("day11a: {}", solve(&floor, step_a));
    println!("day11b: {}", solve(&floor, step_b));
}
