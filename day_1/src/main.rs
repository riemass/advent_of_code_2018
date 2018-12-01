#![feature(duration_as_u128)]
use std::io::{self, BufRead};
use std::collections::{HashSet};

// fn main() {
//     let num: i32 = io::stdin().lock().lines().filter_map(|r| r.ok()).filter_map(|x| x.parse::<i32>().ok()).sum();
//     println!("Frequency is: {}", num);
// }

fn main() {
    let mut set : HashSet<i32> = HashSet::new();

    let nums: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .filter_map(|r| r.ok())
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();

    let sum: i32 = nums.iter().sum();
    println!("Frequency is: {}", sum);

    for x in nums.iter().cycle().scan(0, |state, x| {
        *state = *state + x;
        Some(*state)
    }) {
        if set.contains(&x) {
            println!("First double value is: {}", x);
            break;
        } else {
            set.insert(x);
        }
    }
}
