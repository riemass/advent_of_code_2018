/// register file - 4 registers 0-3
///
/// instruction
/// - opcode
/// - inputs A & B
/// - output C
/// - A vs reguster A
/// - C always register
///  
/// addi 0 7 3 => reg 0 + value 7 -> reg 3
///
/// instructions:
/// addr, addi,
/// mulr, muli,
/// banr, bani,
/// borr, bori,
/// setr, seti,
/// gtir, gtri, gtrr,
/// eqir, eqri, eqrr,
///
/// opcode -> 0-15
///
///
use std::io::{BufRead};

type RegisterFile = [i32; 4];

fn addr(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] + registers[b as usize];
}

fn addi(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] + b;
}

fn mulr(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] * registers[b as usize];
}

fn muli(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] * b;
}

fn banr(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] & registers[b as usize];
}

fn bani(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] & b;
}

fn borr(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] | registers[b as usize];
}

fn bori(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = registers[a as usize] | b;
}

fn setr(registers: &mut RegisterFile, a: i32, _b: i32, c: i32) {
    registers[c as usize] = registers[a as usize];
}

fn seti(registers: &mut RegisterFile, a: i32, _b: i32, c: i32) {
    registers[c as usize] = a;
}

fn gtir(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = (a > registers[b as usize]) as i32;
}

fn gtri(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = (registers[a as usize] > b) as i32;
}

fn gtrr(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = (registers[a as usize] > registers[b as usize]) as i32;
}

fn eqir(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = (a == registers[b as usize]) as i32;
}

fn eqri(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = (registers[a as usize] == b) as i32;
}

fn eqrr(registers: &mut RegisterFile, a: i32, b: i32, c: i32) {
    registers[c as usize] = (registers[a as usize] == registers[b as usize]) as i32;
}

type Instruction = fn(&mut RegisterFile, i32, i32, i32);

struct Opcode {
    opcode: i32,
    reg_a: i32,
    reg_b: i32,
    reg_c: i32, 
}

fn main() {
    let _a = [
        addi, addr, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri,
        eqrr,
    ];

    let input = std::io::stdin();
    let mut lines = input.lock().lines().filter_map(|x| x.ok());

    loop {
        let line = match lines.next() {
            None => break,
            Some(ref line) if line == "" => break,
            Some(line) => line,
        };

    }



    let mut reg_file: RegisterFile = serde_json::from_str("[0, 1, 1, 0]").unwrap();
    let expected: RegisterFile = serde_json::from_str("[0, 1, 1, 1]").unwrap();

    for instruction in _a.iter() {
        instruction(&mut reg_file, 1, 0, 3);
        if reg_file == expected {
            println!{"could be"};
        }
    }
}
