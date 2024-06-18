use std::collections::HashMap;

// const INPUT: [usize; 3] = [3, 1, 2];
const INPUT: [usize; 6] = [1, 0, 15, 2, 10, 13];
const R0: usize = 2020;
const R1: usize = 30000000;
// fn solve<const N: usize>() -> usize {
//     let l = INPUT.len();
//     if N <= l {
//         return INPUT[N - 1];
//     }
//     let mut arr = HashMap::new();
//     for (i, n) in INPUT.iter().enumerate() {
//         arr.insert(*n, i + 1);
//     }
//     let mut last_spoken = 0;
//     for c in (l + 1)..N {
//         let tmp = last_spoken;
//         if let Some(n) = arr.get_mut(&last_spoken) {
//             last_spoken = c - *n;
//         } else {
//             last_spoken = 0;
//         }
//         arr.insert(tmp, c);
//     }
//     last_spoken
// }
fn solve<const N: usize>() -> usize {
    let l = INPUT.len();
    if N <= l {
        return INPUT[N - 1];
    }
    let mut arr = vec![0; N];
    for (i, n) in INPUT.iter().enumerate() {
        arr[*n] = i + 1;
    }
    let mut last_spoken = 0;
    for c in (l + 1)..N {
        let n = arr[last_spoken];
        arr[last_spoken] = c;
        if n == 0 {
            last_spoken = 0;
        } else {
            last_spoken = c - n;
        }
    }
    last_spoken
}
pub fn run(_day: usize) {
    println!("day15a: {}", solve::<R0>());
    println!("day15b: {}", solve::<R1>());
}
