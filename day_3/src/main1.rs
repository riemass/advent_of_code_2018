use std::cmp::{max, min};
use std::io::{self, BufRead};
use std::iter::Sum;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Default, Debug, Copy, Clone)]
struct Rect {
    left: i64,
    top: i64,
    right: i64,
    bottom: i64,
}

impl Rect {
    fn area(&self) -> i64 {
        (self.bottom - self.top + 1) * (self.right - self.left + 1)
    }
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

        Ok(r)
    }
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

fn main() {
    let num: Vec<Rect> = io::stdin()
        .lock()
        .lines()
        .filter_map(|r| r.ok())
        .filter_map(|x| x.parse::<Rect>().ok())
        .collect();

    let intersections = all_intersections(&num);

    let mut s: i64 = intersections.iter().map(|r| r.area()).sum();

    for i in 0..intersections.len() {
        let c = num
            .iter()
            .filter(|r| intersect(&intersections[i], &r))
            .count();
        if c > 2 {
            s -= (c as i64 - 2) * intersections[i].area();
        }
    }

    println!("{:?}", s);

    // let mut iter_1 = num[1..num.len()].iter();
    // let mut iter_2 = num[0..num.len() - 1].iter();
    // let mut viter = num.iter();
    // let mut seciter = num.iter();
    // seciter.next();
}
