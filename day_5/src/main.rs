#![feature(duration_as_u128)]
use std::io::{self, BufRead};

fn check(i: u8, j: u8) -> bool {
    match i as i8 - j as i8 {
        32 | -32 => true,
        _ => false,
    }
}

fn react(mut ch: Vec<u8>) -> usize {

    loop {
        for i in 0..(ch.len() - 1) {
            if check(ch[i], ch[i + 1]) {
                ch[i] = ' ' as u8;
                ch[i + 1] = ' ' as u8;
            }
        }
        let old_len = ch.len();
        ch.retain(|&x| x != ' ' as u8);
        let retained = (old_len - ch.len()) != 0;
        if !retained {
            break;
        }
    }

    return ch.len();

}

fn main() {
    let mut s = String::new();
    io::stdin().lock().read_line(&mut s).unwrap();
    let ch: Vec<u8> = Vec::from(s.trim());

    let mut lens = Vec::new();

    for c in ('a' as u8)..('z' as u8) {
        let mut ch = ch.clone();
        ch.retain(|&f| f != c && f != c - 32);
        lens.push(react(ch));
    }

    println!("{:?}", lens.iter().min());
}
