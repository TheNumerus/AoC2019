use crate::day_2::*;
use std::io::{Cursor, Write, BufRead};

pub fn solve(is_part_two: bool) {
    let program = generate_program(INPUT_ROBOT);
    let input = Cursor::new(Vec::new());
    let output = Cursor::new(Vec::new());
    let mut robot = IntCodeComputer::new(program, input, output);
    let mut dir = Dir::Top;
    let (mut x, mut y) = (0,0);
    let mut hull = Hull::new(u32::from(is_part_two));
    while !robot.halted {
        let (color, turn) = robot_step(hull.get_hull_color_by_pos(x, y), &mut robot);
        hull.paint(x, y, color);
        dir.turn(turn);
        match dir {
            Dir::Top => y += 1,
            Dir::Left => x -= 1,
            Dir::Right => x += 1,
            Dir::Down => y -= 1,
        }
    }
    dbg!(hull.hull.len());
    hull.render();
}

fn robot_step(color: u32, robot: &mut IntCodeComputer<Cursor<Vec<u8>>, Cursor<Vec<u8>>>) -> (u32, u32) {
    robot.input.set_position(0);
    robot.input.write(format!("{}\n", color).as_bytes()).unwrap();
    robot.input.set_position(0);
    robot.run_interrupt();
    let mut out = String::new();
    robot.output.set_position(0);
    robot.output.read_line(&mut out).unwrap();
    let color: u32 = out.trim().parse().unwrap();
    out.clear();
    robot.output.read_line(&mut out).unwrap();
    let dir: u32 = out.trim().parse().unwrap();
    robot.output.set_position(0);
    (color, dir)
}

struct Hull {
    hull: Vec<(i32, i32, u32)>
}

impl Hull {
    fn new(start_color: u32) -> Self {
        let mut hull = Vec::new();
        hull.push((0, 0, start_color));
        Self{hull}
    }

    fn get_hull_color_by_pos(&self, x: i32, y: i32) -> u32 {
        for (x_hull, y_hull, color) in &self.hull {
            if x == *x_hull && y == *y_hull {
                return *color;
            }
        }
        0
    }

    fn paint(&mut self, x: i32, y: i32, color: u32) {
        for (x_hull, y_hull, color_hull) in &mut self.hull {
            if x == *x_hull && y == *y_hull {
                *color_hull = color;
                return;
            }
        }
        self.hull.push((x, y, color));
    }

    fn render(&self) {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        for (x_hull, y_hull, _) in &self.hull {
            if *x_hull < min_x {
                min_x = *x_hull;
            }
            if *x_hull > max_x {
                max_x = *x_hull;
            }
            if *y_hull < min_y {
                min_y = *y_hull;
            }
            if *y_hull > max_y {
                max_y = *y_hull;
            }
        }
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let color = if self.get_hull_color_by_pos(x,y) == 1 {
                    '█'
                } else {
                    ' '
                };
                print!("{}", color);
            }
            println!();
        }
    }
}

enum Dir {
    Left,
    Top,
    Right,
    Down
}

impl Dir {
    fn turn(&mut self, dir: u32){
        match self {
            Dir::Left =>{
                if dir == 0 {
                    *self = Dir::Down;
                } else {
                    *self = Dir::Top;
                }
            },
            Dir::Right => {
                if dir == 0 {
                    *self = Dir::Top;
                } else {
                    *self = Dir::Down;
                }
            },
            Dir::Top => {
                if dir == 0 {
                    *self = Dir::Left;
                } else {
                    *self = Dir::Right;
                }
            },
            Dir::Down => {
                if dir == 0 {
                    *self = Dir::Right;
                } else {
                    *self = Dir::Left;
                }
            }
        }
    }
}

const INPUT_ROBOT: &str = "3,8,1005,8,330,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,29,2,9,4,10,1006,0,10,1,1103,17,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,101,0,8,61,1006,0,21,1006,0,51,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,1001,8,0,89,1,102,19,10,1,1107,17,10,1006,0,18,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,1001,8,0,123,1,9,2,10,2,1105,10,10,2,103,9,10,2,1105,15,10,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,102,1,8,161,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,182,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,101,0,8,205,2,1102,6,10,1006,0,38,2,1007,20,10,2,1105,17,10,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,241,3,8,102,-1,8,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,263,1006,0,93,2,5,2,10,2,6,7,10,3,8,102,-1,8,10,101,1,10,10,4,10,108,0,8,10,4,10,1001,8,0,296,1006,0,81,1006,0,68,1006,0,76,2,4,4,10,101,1,9,9,1007,9,1010,10,1005,10,15,99,109,652,104,0,104,1,21102,825594262284,1,1,21102,347,1,0,1105,1,451,21101,0,932855939852,1,21101,358,0,0,1106,0,451,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,1,235152649255,1,21101,405,0,0,1105,1,451,21102,235350879235,1,1,21102,416,1,0,1106,0,451,3,10,104,0,104,0,3,10,104,0,104,0,21102,988757512972,1,1,21101,439,0,0,1106,0,451,21102,1,988669698828,1,21101,0,450,0,1106,0,451,99,109,2,22101,0,-1,1,21102,40,1,2,21102,1,482,3,21102,472,1,0,1106,0,515,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,477,478,493,4,0,1001,477,1,477,108,4,477,10,1006,10,509,1101,0,0,477,109,-2,2106,0,0,0,109,4,1202,-1,1,514,1207,-3,0,10,1006,10,532,21102,1,0,-3,21202,-3,1,1,21202,-2,1,2,21102,1,1,3,21102,1,551,0,1106,0,556,109,-4,2105,1,0,109,5,1207,-3,1,10,1006,10,579,2207,-4,-2,10,1006,10,579,22101,0,-4,-4,1105,1,647,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,598,1,0,1105,1,556,21202,1,1,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,617,21102,1,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,639,21202,-1,1,1,21102,1,639,0,105,1,514,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0";