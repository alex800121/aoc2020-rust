use aoc2020::{Clockwise, ZipWith};
use itertools::{Either, Itertools};
use project_root::get_project_root;
use std::{array::from_fn, collections::BTreeMap};

const N: usize = 10;
type Tile<const N: usize> = [[bool; N]; N];
type NumTile<const N: usize> = (u64, Tile<N>);
type Index = [i64; 2];
type Stitched<const M: usize> = BTreeMap<Index, NumTile<M>>;
type Final = [[bool; 8 * 12]; 8 * 12];

const DRAGON: &str = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";
type Dragon = Vec<Vec<bool>>;
fn find_dragon(mut fin: Final) -> usize {
    let mut dragon: Dragon = DRAGON
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect_vec())
        .collect_vec();
    for _ in 0..2 {
        for _ in 0..4 {
            for x in 0..(8 * 12 - dragon[0].len()) {
                for y in 0..(8 * 12 - dragon.len()) {
                    if dragon.iter().enumerate().all(|(y0, l)| {
                        l.iter().enumerate().all(|(x0, b)| {
                            if *b {
                                fin[y + y0][x + x0]
                            } else {
                                true
                            }
                        })
                    }) {
                        dragon.iter().enumerate().for_each(|(y0, l)| {
                            l.iter().enumerate().for_each(|(x0, b)| {
                                if *b {
                                    fin[y + y0][x + x0] = false;
                                }
                            })
                        })
                    }
                }
            }
            dragon = dragon.clockwise();
        }
        dragon.reverse();
    }
    fin.iter()
        .map(|x| x.iter().filter(|x| **x).count())
        .sum::<usize>()
}
fn build_final(s: Stitched<N>) -> Final {
    let [x0, y0] = s.keys().min().unwrap();
    from_fn(|y| {
        from_fn(|x| {
            let xm = x % 8 + 1;
            let xs = x / 8;
            let ym = y % 8 + 1;
            let ys = y / 8;
            s.get(&[xs as i64 + x0, ys as i64 + y0]).unwrap().1[ym][xm]
        })
    })
}
fn stitch_all<const N: usize>(mut ts: Vec<NumTile<N>>) -> Option<Stitched<N>> {
    let mut output = BTreeMap::new();
    let mut next_tiles = vec![([0, 0], ts.pop()?)];
    while !next_tiles.is_empty() {
        let (a, b): (Vec<_>, Vec<_>) = ts.into_iter().partition_map(|mut x| {
            let mut ns = next_tiles.iter();
            'a: loop {
                if let Some(t0) = ns.next() {
                    match stitch_tile(&t0.1 .1, &mut x.1) {
                        None => {
                            continue 'a;
                        }
                        Some(y) => {
                            break 'a Either::Left((y.0.zip_with(|a, b| a + b, t0.0), (x.0, y.1)));
                        }
                    }
                } else {
                    break 'a Either::Right(x);
                }
            }
        });
        output.extend(next_tiles.drain(..));
        next_tiles.extend(a);
        ts = b;
    }
    Some(output)
}
fn stitch_tile<const N: usize>(t0: &Tile<N>, t1: &mut Tile<N>) -> Option<(Index, Tile<N>)> {
    for _ in 0..2 {
        for _ in 0..4 {
            for ([x, y], [s0, s1]) in [
                ([1, 0], [9, 0]),
                ([0, 1], [9, 0]),
                ([-1, 0], [0, 9]),
                ([0, -1], [0, 9]),
            ] {
                if (y == 0 && (0..10).all(|i| t0[i][s0] == t1[i][s1]))
                    || (x == 0 && (0..10).all(|i| t0[s0][i] == t1[s1][i]))
                {
                    return Some(([x, y], *t1));
                }
            }
            *t1 = t1.clockwise();
        }
        t1.reverse();
    }
    None
}
fn parse_numtile(input: &str) -> Option<NumTile<N>> {
    let input = input.trim_start_matches("Tile ");
    let (no, tile) = input.split_once(":\n")?;
    let tile = tile
        .lines()
        .filter_map(|x| x.chars().map(|y| y == '#').collect_vec().try_into().ok())
        .collect_vec()
        .try_into()
        .ok()?;
    Some((no.parse::<u64>().ok()?, tile))
}
pub fn run(day: usize) {
    let input = std::fs::read_to_string(format!(
        "{}/input/input{:02}.txt",
        get_project_root().unwrap().to_str().unwrap(),
        day
    ))
    .unwrap();
    let input = input.split("\n\n").filter_map(parse_numtile).collect_vec();
    let ans = stitch_all(input).unwrap();
    let ([x0, y0], [x1, y1]) = ans.keys().minmax().into_option().unwrap();
    println!(
        "day20a: {}",
        [x0, x1]
            .iter()
            .map(|x| [y0, y1]
                .iter()
                .filter_map(|y| ans.get(&[**x, **y]).map(|z| z.0))
                .product::<u64>())
            .product::<u64>()
    );
    println!("day20b: {}", find_dragon(build_final(ans)));
}
