#![feature(vec_resize_with)]

use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::str::FromStr;
use std::cmp::{min, max};

#[derive(Default, Debug, Clone)]
struct Rect {
    id: String,
    left: i64,
    top: i64,
    right: i64,
    bottom: i64,
}

fn intersection(f: &Rect, s: &Rect) -> Rect {
    let mut r = Rect::default();
    r.top = max(f.top, s.top);
    r.bottom = min(f.bottom, s.bottom);
    r.left = max(f.left, s.left);
    r.right = min(f.right, s.right);
    r
}

fn intersect(f: &Rect, s: &Rect) -> bool {
    return !(f.bottom < s.top || s.bottom < f.top || f.left > s.right || s.left > f.right);
}

fn all_intersections(num: &Vec<Rect>) -> Vec<Rect> {
    let mut intersections: Vec<Rect> = Vec::default();
    for i in 0..num.len() {
        for j in i + 1..num.len() {
            if intersect(&num[i], &num[j]) {
                intersections.push(intersection(&num[i], &num[j]));
            }
        }
    }
    intersections
}

impl FromStr for Rect {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let dimensions: Vec<&str> = parts[2]
            .split(|c| c == ',' || c == ':')
            .filter(|s| !s.is_empty())
            .collect();

        let mut r = Rect::default();
        r.left = dimensions[0].parse::<i64>()?;
        r.top = dimensions[1].parse::<i64>()?;

        let dimensions: Vec<&str> = parts[3].split("x").collect();
        r.right = r.left + dimensions[0].parse::<i64>()? - 1;
        r.bottom = r.top + dimensions[1].parse::<i64>()? - 1;

        r.id = parts[0].to_owned();

        Ok(r)
    }
}


fn main() {
    let rects: Vec<Rect> = io::stdin()
        .lock()
        .lines()
        .filter_map(|r| r.ok())
        .filter_map(|x| x.parse::<Rect>().ok())
        .collect();

    let max_y = rects.iter().map(|r| r.bottom).max().unwrap() as usize;
    let max_x = rects.iter().map(|r| r.right).max().unwrap() as usize;

    let mut fabric: Vec<Vec<i32>> = Vec::new();
    fabric.resize_with(max_y + 1, || {
        let mut x: Vec<i32> = Vec::new();
        x.resize(max_x + 1, 0);
        x
    });

    for rect in rects.iter() {
        for i in rect.top..rect.bottom + 1 {
            for j in rect.left..rect.right + 1 {
                fabric[i as usize][j as usize] += 1;
            }
        }
    }

    let c: usize = fabric
        .iter()
        .map(|x| x.iter().filter(|c| **c > 1).count())
        .sum();
    println!("{:?}", c);

    let intersections = all_intersections(&rects);

    let good = rects.iter().find(|x| intersections.iter().filter(|y| intersect(x, y)).count() == 0);
    if let Some(f) = good {
        println!("Evo ga: {:?}", f);
    }

    // let mut iter_1 = num[1..num.len()].iter();
    // let mut iter_2 = num[0..num.len() - 1].iter();
    // let mut viter = num.iter();
    // let mut seciter = num.iter();
    // seciter.next();
}
