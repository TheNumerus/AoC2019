pub fn solve() {
    let asteroids = gen_asteroids();
    let mut max = 0;
    let mut max_coords = Point{x: 0, y: 0};
    for (i, center) in asteroids.iter().enumerate() {
        let mut dirs = Vec::new();
        for (other_i, other) in asteroids.iter().enumerate() {
            if i != other_i {
                //println!("{}X{}", center.x - other.x, center.y - other.y);
                let gcd = gcd(center.x - other.x, center.y - other.y);
                //dbg!(center.x - other.x, center.y - other.y, gcd);
                let point = Point{x: (center.x - other.x) / gcd, y:  (center.y - other.y) / gcd};
                if !dirs.contains(&point) {
                    dirs.push(point);
                } else {
                    //dbg!(center.x - other.x, center.y - other.y, gcd);
                }
            }
        }
        if dirs.len() > max {
            max = dirs.len();
            max_coords = *center;
        }
    }
    println!("{}, {:?}", max, max_coords);
    let mut offsets = get_offsets(max_coords, &asteroids);
    offsets.sort_by( |a, b| {
        //dbg!(a,b);
        let (a_angle, a_len) = a.get_angle_len();
        let (b_angle, b_len) = b.get_angle_len();
        if (a_angle - b_angle).abs() < 0.0001 {
            a_len.partial_cmp(&b_len).unwrap()
        } else {
            a_angle.partial_cmp(&b_angle).unwrap()
        }
    });
    dbg!(&offsets[0..10]);
}


fn gen_asteroids() -> Vec<Point> {
    let mut asteroids = Vec::new();
    for (y, line) in INPUT_MAP.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => asteroids.push(Point{x: x as i32,y: y as i32}),
                _ => {}
            }
        }
    }
    asteroids
}

fn get_offsets(center: Point, asteroids: &[Point]) -> Vec<Point3> {
    let mut offsets = Vec::new();
    for (i, asteroid) in asteroids.iter().enumerate() {
        if center != *asteroid {
            offsets.push(Point3{x: asteroid.x - center.x, y: asteroid.y - center.y, z: i});
        }
    }
    offsets
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b.abs();
    }
    if b == 0 {
        return a.abs();
    }
    let a = a.abs();
    let b = b.abs();
    for i in (1..=(a.min(b))).rev() {
        if a % i == 0 && b % i == 0 {
            return i;
        }
    }
    return 1;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point3 {
    x: i32,
    y: i32,
    z: usize
}

impl Point3 {
    fn get_angle_len(&self) -> (f32, f32) {
        let angle = if self.x == 0 {
            if self.y > 0 {
                std::f32::consts::PI
            } else {
                0.0
            }
        } else if self.y == 0 {
            if self.x > 0 {
                std::f32::consts::FRAC_PI_2
            } else {
                std::f32::consts::FRAC_PI_2 + std::f32::consts::PI
            }
        } else if self.x > 0 && self.y > 0 {
            (self.y as f32 / self.x as f32).atan() + std::f32::consts::FRAC_PI_2 // 2nd quadrant
        } else if self.x < 0 && self.y > 0 {
            (self.y as f32 / -self.x as f32).atan() + std::f32::consts::PI // 3rd quadrant
        } else if self.x > 0 && self.y < 0 {
            (-self.y as f32 / self.x as f32).atan() // 1st quadrant
        } else {
            (-self.y as f32 / -self.x as f32).atan() + std::f32::consts::PI + std::f32::consts::FRAC_PI_2 // 4th quadrant
        };

        let len = ((self.x as f32).powi(2) + (self.y as f32).powi(2)).sqrt();
        (angle, len)
    }
}

const INPUT_MAP: &str =
".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
//const INPUT_MAP: &str = include_str!("input_10");