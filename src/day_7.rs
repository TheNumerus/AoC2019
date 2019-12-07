use crate::day_2::*;
use std::io::{Cursor, Read, Write, Seek, SeekFrom};
use std::ops::Range;

pub fn solve() {
    let mut max_out = 0;
    let mut max_combo = 0;
    let combos = gen_phase_combos(0..5);
    for (index, combo) in combos.iter().enumerate() {
        let mut output = String::from("0\n");
        for i in combo {
            let mut amp = new_amp(*i, output.as_str());
            amp.run();
            read_cursor_to_string(&mut amp.output, &mut output);
        }
        let num = output.trim().parse().unwrap();
        if num > max_out {
            max_out = num;
            max_combo = index;
        }
    }
    println!("max combo = {:?}, max_out = {}", combos[max_combo], max_out);
}

pub fn solve_part_2() {
    let mut max_out = 0;
    let mut max_combo = 0;
    let combos = gen_phase_combos(5..10);
    for (index, combo) in combos.iter().enumerate() {
        let mut output = String::from("0\n");
        let mut amps = Vec::new();
        for i in 0..5 {
            let mut amp = new_amp(combo[i], output.as_str());
            amp.run_interrupt();
            read_cursor_to_string(&mut amp.output, &mut output);
            amp.output.get_mut().clear();
            amp.output.seek(SeekFrom::Start(0)).unwrap();
            amps.push(amp);
        }

        loop {
            for amp in &mut amps {
                set_cursor_start_value(output.as_str(), &mut amp.input);
                amp.run_interrupt();
                read_cursor_to_string(&mut amp.output, &mut output);
                amp.output.seek(SeekFrom::Start(0)).unwrap();
                amp.output.get_mut().clear();
            }
            if amps[4].halted {
                break;
            }
        }
        let num = output.trim().split('\n').nth_back(0).unwrap().trim().parse().unwrap();
        if num > max_out {
            max_out = num;
            max_combo = index;
        }
    }
    println!("max combo = {:?}, max_out = {}", combos[max_combo], max_out);
}

fn gen_phase_combos(range: Range<u32>) -> Vec<[u32;5]> {
    let mut out = Vec::new();
    for a in range.clone() {
        for b in range.clone() {
            if a == b {
                continue;
            }
            for c in range.clone() {
                if a == c || b == c {
                    continue;
                }
                for d in range.clone() {
                    if a == d || b == d || c == d {
                        continue;
                    }
                    for e in range.clone() {
                        if a == e || b == e || c == e || d == e {
                            continue;
                        }
                        out.push([a,b,c,d,e]);
                    }
                }
            }
        }
    }
    out
}

fn set_cursor_start_value(value: &str, cursor: &mut Cursor<Vec<u8>>) {
    cursor.seek(SeekFrom::Start(0)).unwrap();
    cursor.write(value.as_bytes()).unwrap();
    cursor.seek(SeekFrom::Start(0)).unwrap();
}

fn read_cursor_to_string(cursor: &mut Cursor<Vec<u8>>, string: &mut String) {
    cursor.seek(SeekFrom::Start(0)).unwrap();
    string.clear();
    cursor.read_to_string(string).unwrap();
}

fn new_amp(phase: u32, last_amp: &str) -> IntCodeComputer<Cursor<Vec<u8>>, Cursor<Vec<u8>>> {
    let mut input = Cursor::new(Vec::new());
    let output = Cursor::new(Vec::new());
    let amp_input = format!("{}\n{}", phase, last_amp);
    //dbg!(&amp_input);
    set_cursor_start_value(amp_input.as_str(), &mut input);
    let program = generate_program(INPUT_AMP);
    IntCodeComputer::new(program, input, output)
}

//const INPUT_AMP: &str = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
//const INPUT_AMP: &str = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
const INPUT_AMP: &str = "3,8,1001,8,10,8,105,1,0,0,21,38,63,88,97,118,199,280,361,442,99999,3,9,1002,9,3,9,101,2,9,9,1002,9,4,9,4,9,99,3,9,101,3,9,9,102,5,9,9,101,3,9,9,1002,9,3,9,101,3,9,9,4,9,99,3,9,1002,9,2,9,1001,9,3,9,102,3,9,9,101,2,9,9,1002,9,4,9,4,9,99,3,9,102,2,9,9,4,9,99,3,9,102,4,9,9,101,5,9,9,102,2,9,9,101,5,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,1,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,99";