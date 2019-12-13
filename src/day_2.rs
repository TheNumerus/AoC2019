use std::convert::{TryFrom, TryInto};
use std::io::{stdin, stdout, Read, Write, BufRead};

pub fn solve_params(program: &str, par_1: isize, par_2: isize) {
    let mut program = generate_program(program);
    program[1] = par_1;
    program[2] = par_2;
    let stdin = stdin();
    let mut cpu = IntCodeComputer::new(program, stdin.lock(), stdout());
    cpu.run();
    println!("program[0] = {}", cpu.memory[0]);
}

pub fn generate_program(program: &str) -> Vec<isize> {
    program.split(",").map(|num| num.parse().unwrap()).collect()
}

pub struct IntCodeComputer<I: Read + BufRead, O: Write> {
    pc: usize,
    memory: Vec<isize>,
    relative_base: isize,
    pub halted: bool,
    pub input: I,
    pub output: O,
}

impl<I: Read + BufRead, O: Write> IntCodeComputer<I, O> {
    pub fn new(memory: Vec<isize>, input: I, output: O) -> IntCodeComputer<I, O> {
        IntCodeComputer{pc: 0, memory, halted: false, input, output, relative_base: 0}
    }

    pub fn run(&mut self) {
        use Instr::*;
        let mut instr = self.fetch_decode();
        while !self.halted {
            match instr {
                Halt => {
                    self.halt();
                },
                Add(first_dir, second_dir, target) => self.add(first_dir, second_dir, target),
                Mul(first_dir, second_dir, target) => self.mul(first_dir, second_dir, target),
                Read(direct) => self.read(direct),
                Write(direct) => self.write(direct),
                JmpNonZero(check, target) => self.jmp_non_zero(check, target),
                JmpZero(check, target) => self.jmp_zero(check, target),
                LessThan(first_dir, second_dir, target) => self.less_than(first_dir, second_dir, target),
                Eq(first_dir , second_dir , target) => self.eq(first_dir, second_dir, target),
                SetBase(direct) => self.set_base(direct)
            }
            instr = self.fetch_decode();
        }
    }

    pub fn run_interrupt(&mut self) -> ReturnSignal {
        use Instr::*;
        let mut instr = self.fetch_decode();
        while !self.halted {
            match instr {
                Halt => {
                    self.halt();
                },
                Add(first_dir, second_dir, target) => self.add(first_dir, second_dir, target),
                Mul(first_dir, second_dir, target) => self.mul(first_dir, second_dir, target),
                Read(direct) => {
                    if self.read_interrupt(direct) {
                        return ReturnSignal::Interrupt;
                    }
                },
                Write(direct) => self.write(direct),
                JmpNonZero(check, target) => self.jmp_non_zero(check, target),
                JmpZero(check, target) => self.jmp_zero(check, target),
                LessThan(first_dir, second_dir, target) => self.less_than(first_dir, second_dir, target),
                Eq(first_dir , second_dir , target) => self.eq(first_dir, second_dir, target),
                SetBase(direct) => self.set_base(direct)
            }
            instr = self.fetch_decode();
        }
        ReturnSignal::Halt
    }

    fn halt(&mut self) -> ReturnSignal {
        self.halted = true;
        ReturnSignal::Halt
    }

    fn add(&mut self, first_dir: ParType, second_dir: ParType, target: ParType) {
        let first = self.memory_get(first_dir, 1);
        let second = self.memory_get(second_dir, 2);
        let target = self.memory_get_mut(target, 3);
        *target = first + second;
        self.pc += 4;
    }

    fn mul(&mut self, first_dir: ParType, second_dir: ParType, target: ParType) {
        let first = self.memory_get(first_dir, 1);
        let second = self.memory_get(second_dir, 2);
        let target = self.memory_get_mut(target, 3);
        *target = first * second;
        self.pc += 4;
    }

    fn read(&mut self, direct: ParType) {
        let mut input = String::new();
        self.input.read_line(&mut input).unwrap();
        let input = input.trim().parse().unwrap();
        let target  = match direct {
            ParType::Direct => self.pc + 1,
            ParType::Indirect => self.memory[self.pc + 1] as usize,
            ParType::Relative => (self.relative_base + self.memory[self.pc + 1]) as usize
        };
        self.memory[target] = input;
        self.pc += 2;
    }

    fn read_interrupt(&mut self, direct: ParType) -> bool {
        let mut input = String::new();
        //dbg!("read");
        self.input.read_line(&mut input).unwrap();
        if input.trim().len() == 0 {
            return true;
        }
        let input = input.trim().parse().unwrap();
        let target  = match direct {
            ParType::Direct => self.pc + 1,
            ParType::Indirect => self.memory[self.pc + 1] as usize,
            ParType::Relative => (self.relative_base + self.memory[self.pc + 1]) as usize
        };
        self.memory[target] = input;
        self.pc += 2;
        false
    }

    fn jmp_non_zero(&mut self, check: ParType, target: ParType) {
        let value = self.memory_get(check, 1);
        let target = self.memory_get(target, 2);

        if value != 0 {
            self.pc = target as usize;
        } else {
            self.pc += 3;
        }
    }

    fn jmp_zero(&mut self, check: ParType, target: ParType) {
        let value = self.memory_get(check, 1);
        let target = self.memory_get(target, 2);

        if value == 0 {
            self.pc = target as usize;
        } else {
            self.pc += 3;
        }
    }

    fn less_than(&mut self, first_dir: ParType, second_dir: ParType, target: ParType) {
        let first = self.memory_get(first_dir, 1);
        let second = self.memory_get(second_dir, 2);
        let target = self.memory_get_mut(target, 3);

        if first < second {
            *target = 1;
        } else {
            *target = 0;
        }
        self.pc += 4;
    }

    fn eq(&mut self, first_dir: ParType, second_dir: ParType, target: ParType) {
        let first = self.memory_get(first_dir, 1);
        let second = self.memory_get(second_dir, 2);
        let target = self.memory_get_mut(target, 3);

        if first == second {
            *target = 1;
        } else {
            *target = 0;
        }
        self.pc += 4;
    }

    fn write(&mut self, direct: ParType) {
        let num = self.memory_get(direct, 1);
        self.output.write(format!("{}\n", num).as_bytes()).unwrap();
        self.pc += 2;
    }

    fn fetch_decode(&self) -> Instr {
        self.memory[self.pc].try_into().unwrap()
    }

    fn memory_get(&mut self, direct: ParType, pc_offset: usize) -> isize {
        let mut target = self.pc + pc_offset;
        if (target + 1) >= self.memory.len() {
            self.memory.resize(target + 1, 0);
        }
        match direct {
            ParType::Indirect => {
                target = self.memory[self.pc + pc_offset] as usize;
                if (target + 1) >= self.memory.len() {
                    self.memory.resize(target + 1, 0);
                }
                self.memory[target]
            },
            ParType::Direct => {
                self.memory[target]
            },
            ParType::Relative => {
                target = (self.memory[self.pc + pc_offset] + self.relative_base) as usize;
                if (target + 1) >= self.memory.len() {
                    self.memory.resize(target + 1, 0);
                }
                self.memory[target]
            }
        }
    }

    fn memory_get_mut(&mut self, direct: ParType, pc_offset: usize) -> &mut isize {
        let mut target = self.pc + pc_offset;
        if (target + 1) >= self.memory.len() {
            self.memory.resize(target + 1, 0);
        }
        match direct {
            ParType::Indirect => {
                target = self.memory[self.pc + pc_offset] as usize;
                if (target + 1) >= self.memory.len() {
                    self.memory.resize(target + 1, 0);
                }
                &mut self.memory[target]
            },
            ParType::Direct => {
                &mut self.memory[target]
            },
            ParType::Relative => {
                target = (self.memory[self.pc + pc_offset] + self.relative_base) as usize;
                if (target + 1) >= self.memory.len() {
                    self.memory.resize(target + 1, 0);
                }
                &mut self.memory[target]
            }
        }
    }

    fn set_base(&mut self, direct: ParType) {
        self.relative_base = self.relative_base + self.memory_get(direct, 1);
        self.pc += 2;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instr {
    Add(ParType, ParType, ParType),
    Mul(ParType, ParType, ParType),
    Read(ParType),
    Write(ParType),
    Halt,
    JmpZero(ParType, ParType),
    JmpNonZero(ParType, ParType),
    LessThan(ParType, ParType, ParType),
    Eq(ParType, ParType, ParType),
    SetBase(ParType)
}

#[derive(Debug, Copy, Clone)]
pub enum ParType {
    Indirect,
    Direct,
    Relative
}

impl TryFrom<isize> for ParType {
    type Error = String;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Indirect),
            1 => Ok(Self::Direct),
            2 => Ok(Self::Relative),
            _ => Err(format!("invalid parameter type {}", value))
        }
    }
}

impl TryFrom<isize> for Instr {
    type Error = String;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        let instruction = value % 100;
        let mode_first = ParType::try_from((value % 1000) / 100).unwrap();
        let mode_second = ParType::try_from((value % 10000) / 1000).unwrap();
        let mode_third = ParType::try_from(value / 10000).unwrap();
        match instruction {
            1 => Ok(Self::Add(mode_first, mode_second, mode_third)),
            2 => Ok(Self::Mul(mode_first, mode_second, mode_third)),
            3 => Ok(Self::Read(mode_first)),
            4 => Ok(Self::Write(mode_first)),
            5 => Ok(Self::JmpNonZero(mode_first, mode_second)),
            6 => Ok(Self::JmpZero(mode_first, mode_second)),
            7 => Ok(Self::LessThan(mode_first, mode_second, mode_third)),
            8 => Ok(Self::Eq(mode_first, mode_second, mode_third)),
            9 => Ok(Self::SetBase(mode_first)),
            99 => Ok(Self::Halt),
            _ => Err(format!("invalid instruction {}", value))
        }
    }
}

pub enum ReturnSignal {
    Halt,
    Interrupt
}

pub const INPUT_GRAVITY: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,19,5,23,2,23,6,27,1,27,5,31,2,6,31,35,1,5,35,39,2,39,9,43,1,43,5,47,1,10,47,51,1,51,6,55,1,55,10,59,1,59,6,63,2,13,63,67,1,9,67,71,2,6,71,75,1,5,75,79,1,9,79,83,2,6,83,87,1,5,87,91,2,6,91,95,2,95,9,99,1,99,6,103,1,103,13,107,2,13,107,111,2,111,10,115,1,115,6,119,1,6,119,123,2,6,123,127,1,127,5,131,2,131,6,135,1,135,2,139,1,139,9,0,99,2,14,0,0";