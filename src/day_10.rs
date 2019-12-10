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

/*const INPUT_MAP: &str =
".#..#
.....
#####
....#
...##";*/
const INPUT_MAP: &str = include_str!("input_10");