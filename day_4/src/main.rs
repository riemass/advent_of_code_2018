use std::collections::HashMap;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Timestamp {
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
enum Event {
    StartShift(i32),
    FallAsleep,
    WakeUp,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Ord)]
struct Record {
    ts: Timestamp,
    event: Event,
}

impl FromStr for Event {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let a = input
            .split(|c| c == ' ' || c == '#')
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        match a[0] {
            "Guard" => Ok(Event::StartShift(a[1].parse().unwrap())),
            "falls" => Ok(Event::FallAsleep),
            "wakes" => Ok(Event::WakeUp),
            _ => Err("no way".to_owned()),
        }
    }
}

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let a = input
            .split(|c| c == '[' || c == ']' || c == ' ' || c == '-' || c == ':')
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        Ok(Timestamp {
            month: a[1].parse().unwrap(),
            day: a[2].parse().unwrap(),
            hour: a[3].parse().unwrap(),
            minute: a[4].parse().unwrap(),
        })
    }
}

impl FromStr for Record {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let i = input.find(|c| c == ']').unwrap() + 1;

        Ok(Record {
            ts: input[0..i].parse()?,
            event: input[i..input.len()].parse()?,
        })
    }
}

fn minutes_vector() -> Vec<i32> {
    let mut a = Vec::new();
    a.resize(60, 0);
    a
}

fn main() {
    let mut input = io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse().ok())
        .collect::<Vec<Record>>();
    input.sort();

    let mut guard_sleeps: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut current_guard = -1;
    let mut fallen_asleep: Option<i32> = None;

    for record in input.iter() {
        match record.event {
            Event::StartShift(x) => {
                current_guard = x;
            }
            Event::FallAsleep => {
                if fallen_asleep.is_some() {
                    panic!("Fallen asleep set previously")
                }
                fallen_asleep = Some(record.ts.minute);
            }
            Event::WakeUp => {
                if fallen_asleep.is_none() {
                    panic!("Fallen asleep not set for guard {:?}", current_guard)
                }
                let a = guard_sleeps
                    .entry(current_guard)
                    .or_insert(minutes_vector());
                for i in fallen_asleep.unwrap()..record.ts.minute {
                    a[i as usize] += 1;
                }
                fallen_asleep = None;
            }
        }
    }

    let guard = guard_sleeps
        .iter()
        .max_by(|lhs, rhs| lhs.1.iter().sum::<i32>().cmp(&rhs.1.iter().sum::<i32>()))
        .map(|x| x.0)
        .unwrap();
    let minutes = guard_sleeps[guard].iter().sum::<i32>();
    let max = guard_sleeps[guard].iter().max();
    let min = guard_sleeps[guard].iter().position(|c| *c == *max.unwrap());
    println!(
        "Max sleep guard was {:?}, with {:?} minutes, in minute: {:?} slept {:?} times",
        guard, minutes, min, max
    );
    println!("Result is {}", *guard as usize * min.unwrap());


    let max_freq = guard_sleeps.values().map(|x| x.iter().max().unwrap()).max().unwrap();
    let mut guard = guard_sleeps.iter().filter(|x| x.1.iter().find(|t| *t == max_freq).is_some()).next().unwrap();
    let guid = *guard.0;
    let pos = guard.1.iter().position(|m| m == max_freq).unwrap();
    println!("Result is {:?} x {:?} = {:?}",guid, pos, guid as usize * pos);


}
