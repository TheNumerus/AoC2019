use std::convert::{TryFrom, TryInto};

pub fn solve_params(program: &str, par_1: isize, par_2: isize) {
    let mut program = generate_program(program);
    program[1] = par_1;
    program[2] = par_2;
    let mut cpu = IntCodeComputer::new(program);
    cpu.run();
    println!("program[0] = {}", cpu.memory[0]);
}

pub fn generate_program(program: &str) -> Vec<isize> {
    program.split(",").map(|num| num.parse().unwrap()).collect()
}

pub struct IntCodeComputer {
    pc: usize,
    memory: Vec<isize>
}

impl IntCodeComputer {
    pub fn new(memory: Vec<isize>) -> IntCodeComputer {
        IntCodeComputer{pc: 0, memory}
    }

    pub fn run(&mut self) {
        use Instr::*;
        let mut instr = self.fetch_decode();
        loop {
            match instr {
                Halt => break,
                Add(first_dir, second_dir) => self.add(first_dir, second_dir),
                Mul(first_dir, second_dir) => self.mul(first_dir,  second_dir),
                Read(direct) => self.read(direct),
                Write(direct) => self.write(direct),
                JmpNonZero(check, target) => self.jmp_non_zero(check, target),
                JmpZero(check, target) => self.jmp_zero(check, target),
                LessThan(first_dir, second_dir, target) => self.less_than(first_dir, second_dir, target),
                Eq(first_dir , second_dir , target) => self.eq(first_dir, second_dir, target)
            }
            instr = self.fetch_decode();
        }
    }

    fn add(&mut self, first_dir: bool, second_dir: bool) {
        let target = self.memory[self.pc + 3] as usize;
        let first = self.memory_get(first_dir, 1);
        let second = self.memory_get(second_dir, 2);
        self.memory[target] = first + second;
        self.pc += 4;
    }

    fn mul(&mut self, first_dir: bool, second_dir: bool) {
        let target = self.memory[self.pc + 3] as usize;
        let first = self.memory_get(first_dir, 1);
        let second = self.memory_get(second_dir, 2);
        self.memory[target] = first * second;
        self.pc += 4;
    }

    fn read(&mut self, direct: bool) {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().parse().unwrap();
        let target = if direct {
            self.pc + 1
        } else {
            self.memory[self.pc + 1] as usize
        };
        self.memory[target] = input;
        self.pc += 2;
    }

    fn jmp_non_zero(&mut self, check: bool, target: bool) {
        let value = self.memory_get(check, 1);
        let target = self.memory_get(target, 2);

        if value != 0 {
            self.pc = target as usize;
        } else {
            self.pc += 3;
        }
    }

    fn jmp_zero(&mut self, check: bool, target: bool) {
        let value = self.memory_get(check, 1);
        let target = self.memory_get(target, 2);

        if value == 0 {
            self.pc = target as usize;
        } else {
            self.pc += 3;
        }
    }

    fn less_than(&mut self, first_dir: bool, second_dir: bool, target: bool) {
        let first = self.memory_get(first_dir, 1);
        let second = self.memory_get(second_dir, 2);

        let target = if target {
            self.pc + 3
        } else {
            self.memory[self.pc + 3] as usize
        };

        if first < second {
            self.memory[target] = 1;
        } else {
            self.memory[target] = 0;
        }
        self.pc += 4;
    }

    fn eq(&mut self, first_dir: bool, second_dir: bool, target: bool) {
        let first = self.memory_get(first_dir, 1);
        let second = self.memory_get(second_dir, 2);

        let target = if target {
            self.pc + 3
        } else {
            self.memory[self.pc + 3] as usize
        };

        if first == second {
            self.memory[target] = 1;
        } else {
            self.memory[target] = 0;
        }
        self.pc += 4;
    }

    fn write(&mut self, direct: bool) {
        println!("{}", self.memory_get(direct, 1));
        self.pc += 2;
    }

    fn fetch_decode(&self) -> Instr {
        self.memory[self.pc].try_into().unwrap()
    }

    fn memory_get(&self, direct: bool, pc_offset: usize) -> isize {
        if direct {
            self.memory[self.pc + pc_offset]
        } else {
            self.memory[self.memory[self.pc + pc_offset] as usize]
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instr {
    Add(bool, bool),
    Mul(bool, bool),
    Read(bool),
    Write(bool),
    Halt,
    JmpZero(bool, bool),
    JmpNonZero(bool, bool),
    LessThan(bool, bool, bool),
    Eq(bool, bool, bool)
}

impl TryFrom<isize> for Instr {
    type Error = String;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        let instruction = value % 100;
        let mode_first = ((value % 1000) / 100) == 1;
        let mode_second = ((value % 10000) / 1000) == 1;
        let mode_third = (value / 10000) == 1;
        match instruction {
            1 => Ok(Self::Add(mode_first, mode_second)),
            2 => Ok(Self::Mul(mode_first, mode_second)),
            3 => Ok(Self::Read(mode_first)),
            4 => Ok(Self::Write(mode_first)),
            5 => Ok(Self::JmpNonZero(mode_first, mode_second)),
            6 => Ok(Self::JmpZero(mode_first, mode_second)),
            7 => Ok(Self::LessThan(mode_first, mode_second, mode_third)),
            8 => Ok(Self::Eq(mode_first, mode_second, mode_third)),
            99 => Ok(Self::Halt),
            _ => Err(format!("invalid instruction {}", value))
        }
    }
}


pub const INPUT_GRAVITY: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,19,5,23,2,23,6,27,1,27,5,31,2,6,31,35,1,5,35,39,2,39,9,43,1,43,5,47,1,10,47,51,1,51,6,55,1,55,10,59,1,59,6,63,2,13,63,67,1,9,67,71,2,6,71,75,1,5,75,79,1,9,79,83,2,6,83,87,1,5,87,91,2,6,91,95,2,95,9,99,1,99,6,103,1,103,13,107,2,13,107,111,2,111,10,115,1,115,6,119,1,6,119,123,2,6,123,127,1,127,5,131,2,131,6,135,1,135,2,139,1,139,9,0,99,2,14,0,0";