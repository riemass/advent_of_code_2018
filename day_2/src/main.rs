use std::io::{self, BufRead};

fn dec(ch: char) -> u8 {
    let ch = ch as u8;
    assert!(ch > 96);
    assert!(ch < (96 + 32));
    ch - 97
}

fn enc(u: u8) -> char {
    assert!(u < 32);
    (u + 97) as char
}

fn get_char_count(s: &str) -> Vec<i32> {
    let mut char_count = Vec::new();
    char_count.resize(26, 0);
    for c in s.chars() {
        char_count[dec(c) as usize] += 1;
    }
    char_count
}

struct Checksum(i32, i32);

impl Checksum {
    fn add(&mut self, char_count: &Vec<i32>) {
        if let Some(_) = char_count.iter().find(|x| **x == 2) {
            self.0 += 1;
        }
        if let Some(_) = char_count.iter().find(|x| **x == 3) {
            self.1 += 1;
        }
    }

    fn compute(&self) -> i32 {
        self.0 * self.1
    }
}

fn main() {
    let mut checksum = Checksum(0, 0);
    let mut sums: Vec<Vec<i32>> = Vec::new();
    let lines: Vec<String> = io::stdin().lock().lines().map(|x| x.unwrap()).collect();

    for line in lines.iter() {
        let char_count = get_char_count(&line);
        checksum.add(&char_count);
        sums.push(char_count);
    }

    println!("The checksum is: {}.", checksum.compute());

    for i in 0..sums.len() {
        for j in i + 1..sums.len() {
            let mut zero_iter = sums[i]
                .iter()
                .zip(sums[j].iter())
                .map(|z| z.0 - z.1)
                .filter(|x| *x != 1 && *x != -1);
            let count = zero_iter.clone().count();
            let all_zero = zero_iter.all(|x| x == 0);
            if all_zero && count == 24 {
                println!("Found a candidate");

                let common: String = lines[i]
                    .chars()
                    .zip(lines[j].chars())
                    .filter_map(|z| if z.0 == z.1 { Some(z.0) } else { None })
                    .collect();
                println!("here {}", common);
            }
        }
    }
}
