use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("./src/input_6")?;

    let buffer = BufReader::new(input_file);

    let mut orbits = Vec::new();
    for line in buffer.lines() {
        let line = line.unwrap();
        let parts = line.split(')').collect::<Vec<_>>();
        orbits.push(Orbit{name: parts[1].to_owned(), parent: parts[0].to_owned()})
    }

    let mut indir_orbits =  0;

    for orbit in &orbits {
        if orbit.parent.as_str() == "COM" {
            continue;
        }
        let mut indirs = 0;
        let mut current_parent = &orbit.parent;
        loop {
            if current_parent.as_str() == "COM" {
                break;
            }
            current_parent = search_for_parent(&orbits, current_parent).unwrap();
            indirs +=1 ;
        }
        indir_orbits += indirs;
    }

    dbg!(orbits.len() + indir_orbits);

    Ok(())
}

fn search_for_parent<'a>(orbits: &'a [Orbit], child: &String) -> Option<&'a String> {
    for orbit in orbits {
        if orbit.name.as_str() == child.as_str() {
            return Some(&orbit.parent);
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq)]
struct Orbit {
    name: String,
    parent: String
}