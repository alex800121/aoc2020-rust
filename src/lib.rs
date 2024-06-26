use std::array::{self, from_fn};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::Hash;
use std::mem::swap;
use std::ops::{Bound, Range, RangeBounds};
use std::iter::{IntoIterator, Iterator};

use itertools::Itertools;
use num::Bounded;

pub fn chinese_rem(input: &[(i64, i64)]) -> Option<i64> {
    let b = input.iter().map(|x| x.0).product::<i64>();
    let mut acc = 0;
    for (n, r) in input {
        let n0 = b / n;
        let (x, _, z) = egcd(n0, *n);
        if z != 1 {
            return None;
        }
        acc += x * n0 * r;
    }
    Some(acc.rem_euclid(b))
}
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut r0 = a;
    let mut r1 = b;
    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;
    while r1 != 0 {
        swap(&mut r0, &mut r1);
        swap(&mut s0, &mut s1);
        swap(&mut t0, &mut t1);
        let q = r1.div_euclid(r0);
        r1 = r1.rem_euclid(r0);
        s1 -= s0 * q;
        t1 -= t0 * q;
    }
    (s0, t0, r0)
}
pub fn print_map<E>(
    map: &BTreeMap<(isize, isize), E>,
    to_char: impl Fn(Option<&E>) -> char,
) -> String {
    let min_x = map.keys().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let max_x = map.keys().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
    let min_y = map.keys().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
    let max_y = map.keys().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
    let mut output = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            output.push(to_char(map.get(&(x, y))));
        }
        output.push('\n');
    }
    output
}

pub fn build_map<
    H: Iterator<Item = I>,
    I: Iterator<Item = J>,
    J,
    K: Ord,
    E,
    O: Iterator<Item = (K, E)>,
>(
    input: H,
    to_key_elem: impl Fn((usize, usize), J) -> O,
) -> BTreeMap<K, E> {
    let mut output = BTreeMap::new();
    for (i, row) in input.enumerate() {
        for (j, e) in row.enumerate() {
            for (key, element) in to_key_elem((j, i), e) {
                output.insert(key, element);
            }
        }
    }
    output
}

pub trait Flippable {
    type Flipped;
    fn flip(self) -> Self::Flipped;
}

impl<T, U> Flippable for (T, U) {
    type Flipped = (U, T);
    fn flip(self) -> Self::Flipped {
        (self.1, self.0)
    }
}

pub fn reduce_sorted_range<E, I>(mut ranges: I) -> Vec<Range<E>>
where
    E: PartialEq + Eq + PartialOrd,
    I: Iterator<Item = Range<E>>,
{
    let mut output = Vec::new();
    let mut start: E;
    let mut end: E;
    if let Some(first) = ranges.next() {
        start = first.start;
        end = first.end;
        for i in ranges {
            if i.start > end {
                output.push(start..end);
                start = i.start;
                end = i.end;
            } else if i.end > end {
                end = i.end;
            }
        }
        output.push(start..end);
    }
    output
}

pub trait ZipWith<T, U, V> {
    type Other1;
    type Other2;
    fn zip_with(self, func: impl Fn(&T, &U) -> V, other: Self::Other1) -> Self::Other2;
}

impl<T, U, V, const N: usize> ZipWith<T, U, V> for [T; N] {
    type Other1 = [U; N];
    type Other2 = [V; N];
    fn zip_with(self, func: impl Fn(&T, &U) -> V, other: Self::Other1) -> Self::Other2 {
        from_fn(|i| func(&self[i], &other[i]))
    }
}

impl<T, U, V> ZipWith<T, U, V> for Vec<T> {
    type Other1 = Vec<U>;
    type Other2 = Vec<V>;
    fn zip_with(self, func: impl Fn(&T, &U) -> V, other: Self::Other1) -> Self::Other2 {
        let mut output = Vec::new();
        let mut a = self.iter();
        let mut b = other.iter();
        while let (Some(x), Some(y)) = (a.next(), b.next()) {
            output.push(func(x, y));
        }
        output
    }
}

pub fn bfs<I: Eq + Hash + Clone, U: Clone + Eq + Hash>(
    mut starts: HashMap<I, U>,
    ends: impl Fn(&HashMap<I, U>) -> bool,
    nexts: impl Fn((I, U), &mut HashMap<I, U>) -> HashMap<I, U>,
) -> HashMap<I, U> {
    let mut results = HashMap::from_iter(starts.clone());
    let mut next_starts = HashMap::new();
    while !ends(&starts) {
        for i in starts.drain() {
            next_starts.extend(nexts(i, &mut results));
        }
        starts.extend(next_starts.drain());
    }
    results
}

pub trait EucVec {
    fn overlap(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;
    fn subtract(&self, other: &Self) -> HashSet<Self>
    where
        Self: Sized + Hash;
    fn union(&self, other: &Self) -> HashSet<Self>
    where
        Self: Sized + Hash;
}

impl<Idx: Copy + Ord + Hash, const N: usize> EucVec for [Range<Idx>; N] {
    fn overlap(&self, other: &Self) -> Option<Self> {
        let zipper = self.iter().zip(other);
        let mut output: [Range<Idx>; N] = self.clone();
        for (i, (a, b)) in zipper.enumerate() {
            let x = a.start.max(b.start);
            let y = a.end.min(b.end);
            if x >= y {
                return None;
            } else {
                output[i] = x..y;
            }
        }
        Some(output)
    }

    fn subtract(&self, other: &Self) -> HashSet<Self> {
        if let Some(overlapped) = self.overlap(other) {
            let mut acc = HashSet::new();
            let mut prev = self.clone();
            for i in 0..N {
                let s_start = self[i].start;
                let s_end = self[i].end;
                let o_start = overlapped[i].start;
                let o_end = overlapped[i].end;
                if s_start < o_start {
                    let mut x = prev.clone();
                    x[i] = s_start..o_start;
                    acc.insert(x);
                }
                if s_end > o_end {
                    let mut x = prev.clone();
                    x[i] = o_end..s_end;
                    acc.insert(x);
                }
                prev[i] = o_start..o_end;
            }
            acc
        } else {
            HashSet::from([self.clone()])
        }
    }

    fn union(&self, other: &Self) -> HashSet<Self> {
        let mut acc = self.subtract(other);
        acc.insert(other.clone());
        acc
    }
}

pub trait Transpose {
    fn transpose(self) -> Self::Transposed;
    type Transposed;
}

impl<T: Clone, const N: usize, const M: usize> Transpose for [[T; N]; M] {
    type Transposed = [[T; M]; N];
    fn transpose(self) -> Self::Transposed {
        array::from_fn(|x| array::from_fn(|y| self[y][x].clone()))
    }
}
impl<T> Transpose for Vec<Vec<T>> {
    type Transposed = Vec<Vec<T>>;
    fn transpose(self) -> Self::Transposed {
        let mut new_vec: Vec<Vec<T>> = Vec::new();
        let s = self;
        for mut row in s {
            for i in 0..row.len() {
                let e = row.remove(0);
                match new_vec.get_mut(i) {
                    Some(v) => v.push(e),
                    None => {
                        new_vec.push(vec![e]);
                    }
                }
            }
        }
        new_vec
    }
}

pub trait Clockwise {
    fn clockwise(self) -> Self::Other;
    fn counter_clockwise(self) -> Self::Other;
    type Other;
}
impl<T: Clone, const N: usize, const M: usize> Clockwise for [[T; N]; M] {
    type Other = [[T; M]; N];
    fn clockwise(self) -> Self::Other {
        let mut x = self.clone();
        x.reverse();
        x.transpose()
    }

    fn counter_clockwise(self) -> Self::Other {
        let mut x = self.transpose();
        x.reverse();
        x
    }
}
impl<T> Clockwise for Vec<Vec<T>> {
    type Other = Vec<Vec<T>>;
    fn clockwise(self) -> Self {
        let mut s = self;
        s.reverse();
        s.transpose()
    }
    fn counter_clockwise(self) -> Self {
        let mut x = self.transpose();
        x.reverse();
        x
    }
}

pub fn zip_with<T, S, U, F>(a: Vec<T>, b: Vec<S>, f: F) -> Vec<U>
where
    F: Fn(T, S) -> U,
{
    let a = a.into_iter();
    let mut b = b.into_iter();
    let mut c = Vec::new();
    for x in a {
        if let Some(y) = b.next() {
            c.push(f(x, y));
        } else {
            break;
        }
    }
    c
}

pub trait Enum
where
    Self: Sized,
{
    fn to_int(&self) -> isize;
    fn to_enum(n: isize) -> Self;
    fn succ(&self) -> Self {
        Self::to_enum(self.to_int() + 1)
    }
    fn pred(&self) -> Self {
        Self::to_enum(self.to_int() - 1)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn oppose(&self) -> Self {
        self.succ().succ()
    }
    pub fn to_index(&self) -> [isize; 2] {
        use Direction::*;
        match self {
            North => [0, -1],
            South => [0, 1],
            East => [1, 0],
            West => [-1, 0],
        }
    }
}

impl Bounded for Direction {
    fn min_value() -> Self {
        Self::North
    }

    fn max_value() -> Self {
        Self::West
    }
}
impl RangeBounds<Direction> for Direction {
    fn start_bound(&self) -> std::ops::Bound<&Direction> {
        Bound::Included(&Self::North)
    }

    fn end_bound(&self) -> std::ops::Bound<&Direction> {
        Bound::Included(&Self::West)
    }
}
impl Enum for Direction {
    fn to_int(&self) -> isize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
    fn to_enum(n: isize) -> Self {
        match n % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Turn {
    Right,
    Left,
}

impl Enum for Turn {
    fn to_int(&self) -> isize {
        match self {
            Turn::Right => 0,
            Turn::Left => 1,
        }
    }
    fn to_enum(n: isize) -> Self {
        match n % 2 {
            0 => Turn::Right,
            _ => Turn::Left,
        }
    }
}
pub trait Coord {
    fn adjacent(&self) -> Vec<Box<Self>>;
    fn surround(&self) -> Vec<Box<Self>>;
}
impl<const N: usize> Coord for [i64; N] {
    fn surround(&self) -> Vec<Box<Self>> {
        let tmp = [-1, 0, 1];
        let mut v: Vec<Vec<i64>> = vec![vec![]];
        for _ in 0..N {
            v = v
                .into_iter()
                .flat_map(|x| {
                    tmp.iter().map(move |y| {
                        let mut z = x.clone();
                        z.push(*y);
                        z
                    })
                })
                .collect_vec();
        }
        v.into_iter()
            .filter_map(|x| {
                if x.iter().all(|y| *y == 0) {
                    None
                } else {
                    let mut y: Self = [0; N];
                    for i in 0..N {
                        y[i] = x[i].checked_add(self[i])?;
                    }
                    Some(Box::new(y))
                }
            })
            .collect_vec()
    }
    fn adjacent(&self) -> Vec<Box<Self>> {
        let mut v = Vec::new();
        for i in 0..N {
            for x in [-1, 1i64] {
                if let Some(x) = x.checked_add(self[i]) {
                    let mut s0 = *self;
                    s0[i] = x;
                    v.push(Box::new(s0));
                }
            }
        }
        v
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        assert_eq!(None, [-2..2].overlap(&[2..3]));
        assert_eq!(None, [3..4].overlap(&[-2..2]));
        assert_eq!(Some([3..4]), [-2..15].overlap(&[3..4]));
        assert_eq!(Some([3..15]), [-2..15].overlap(&[3..19]));
        assert_eq!(Some([-2..10]), [-2..15].overlap(&[-5..10]));
        assert_eq!(
            Some([-2..10, -56..-40]),
            [-2..15, -56..-10].overlap(&[-5..10, -76..-40])
        );
        assert_eq!(
            Some([-2..10, -56..-40, 1000..1001]),
            [-2..15, -56..-10, 1000..1001].overlap(&[-5..10, -76..-40, 0..10000])
        );
        assert_eq!(
            None,
            [-2..15, -56..-10, 1000..1001].overlap(&[-5..-2, -76..-40, 0..10000])
        );
        assert_eq!(
            None,
            [-2..15, -56..-10, 1000..1001].overlap(&[-5..20, -76..-56, 0..10000])
        );
        assert_eq!(
            None,
            [-2..15, -56..-10, -1003..-1001].overlap(&[-5..20, -76..-40, 0..10000])
        );
    }

    #[test]
    fn test_subtract() {
        assert_eq!(HashSet::from([[-5..-3]]), [-5..1].subtract(&[-3..3]));
        assert_eq!(HashSet::from([[-1..1]]), [-5..1].subtract(&[-6..-1]));
        assert_eq!(
            HashSet::from([[-5..-3], [1..16]]),
            [-5..16].subtract(&[-3..1])
        );
        assert_eq!(HashSet::from([]), [-2..1].subtract(&[-3..1]));
        assert_eq!(HashSet::from([[-2..2]]), [-2..2].subtract(&[3..5]));
        assert_eq!(
            HashSet::from([[-2..2, -2..2]]),
            [-2..2, -2..2].subtract(&[3..5, -1..1])
        );
        assert_eq!(
            HashSet::from([
                [-2..-1, -2..2, -2..2],
                [1..2, -2..2, -2..2],
                [-1..1, -2..-1, -2..2],
                [-1..1, 1..2, -2..2],
                [-1..1, -1..1, -2..-1],
                [-1..1, -1..1, 1..2]
            ]),
            [-2..2, -2..2, -2..2].subtract(&[-1..1, -1..1, -1..1])
        );
        assert_eq!(
            HashSet::from([
                [-2..-1, -2..2, -2..2],
                [1..2, -2..2, -2..2],
                [-1..1, -2..-1, -2..2],
                [-1..1, -1..2, -2..-1],
                [-1..1, -1..2, 1..2],
            ]),
            [-2..2, -2..2, -2..2].subtract(&[-1..1, -1..3, -1..1])
        );
        assert_eq!(
            HashSet::from([
                [-2..-1, -2..2, -2..2],
                [1..2, -2..2, -2..2],
                [-1..1, -2..2, -2..-1],
                [-1..1, -2..2, 1..2],
            ]),
            [-2..2, -2..2, -2..2].subtract(&[-1..1, -3..3, -1..1])
        );
        assert_eq!(
            HashSet::from([
                [-2..-1, -2..2, -2..2],
                [1..2, -2..2, -2..2],
                [-1..1, -2..2, 1..2],
            ]),
            [-2..2, -2..2, -2..2].subtract(&[-1..1, -3..3, -3..1])
        );
        assert_eq!(
            HashSet::from([[-2..-1, -2..2, -2..2], [-1..2, -2..2, 1..2],]),
            [-2..2, -2..2, -2..2].subtract(&[-1..3, -3..3, -3..1])
        );
        assert_eq!(
            HashSet::from([[-2..2, -2..2, 1..2],]),
            [-2..2, -2..2, -2..2].subtract(&[-3..3, -3..3, -3..1])
        );
        assert_eq!(
            HashSet::from([]),
            [-2..2, -2..2, -2..2].subtract(&[-3..3, -3..3, -3..3])
        );
    }

    #[test]
    fn test_union() {
        assert_eq!(HashSet::from([[-5..-3], [-3..3]]), [-5..1].union(&[-3..3]));
        assert_eq!(HashSet::from([[-1..1], [-6..-1]]), [-5..1].union(&[-6..-1]));
        assert_eq!(
            HashSet::from([[-5..-3], [1..16], [-3..1]]),
            [-5..16].union(&[-3..1])
        );
        assert_eq!(HashSet::from([[-3..1]]), [-2..1].union(&[-3..1]));
        assert_eq!(
            HashSet::from([
                [-2..-1, -2..2, -2..2],
                [1..2, -2..2, -2..2],
                [-1..1, -2..-1, -2..2],
                [-1..1, 1..2, -2..2],
                [-1..1, -1..1, -2..-1],
                [-1..1, -1..1, 1..2],
                [-1..1, -1..1, -1..1]
            ]),
            [-2..2, -2..2, -2..2].union(&[-1..1, -1..1, -1..1])
        );
        assert_eq!(
            HashSet::from([
                [-2..-1, -2..2, -2..2],
                [1..2, -2..2, -2..2],
                [-1..1, -2..-1, -2..2],
                [-1..1, -1..2, -2..-1],
                [-1..1, -1..2, 1..2],
                [-1..1, -1..3, -1..1]
            ]),
            [-2..2, -2..2, -2..2].union(&[-1..1, -1..3, -1..1])
        );
        assert_eq!(
            HashSet::from([
                [-2..-1, -2..2, -2..2],
                [1..2, -2..2, -2..2],
                [-1..1, -2..2, -2..-1],
                [-1..1, -2..2, 1..2],
                [-1..1, -3..3, -1..1]
            ]),
            [-2..2, -2..2, -2..2].union(&[-1..1, -3..3, -1..1])
        );
        assert_eq!(
            HashSet::from([
                [-2..-1, -2..2, -2..2],
                [1..2, -2..2, -2..2],
                [-1..1, -2..2, 1..2],
                [-1..1, -3..3, -3..1]
            ]),
            [-2..2, -2..2, -2..2].union(&[-1..1, -3..3, -3..1])
        );
        assert_eq!(
            HashSet::from([
                [-2..-1, -2..2, -2..2],
                [-1..2, -2..2, 1..2],
                [-1..3, -3..3, -3..1],
            ]),
            [-2..2, -2..2, -2..2].union(&[-1..3, -3..3, -3..1])
        );
        assert_eq!(
            HashSet::from([[-2..2, -2..2, 1..2], [-3..3, -3..3, -3..1],]),
            [-2..2, -2..2, -2..2].union(&[-3..3, -3..3, -3..1])
        );
        assert_eq!(
            HashSet::from([[-3..3, -3..3, -3..3]]),
            [-2..2, -2..2, -2..2].union(&[-3..3, -3..3, -3..3])
        );
    }
}
