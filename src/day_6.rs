use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::usize;

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

    find_nca("SAN", "YOU", &orbits);

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

fn find_nca(first: &str, second: &str, orbits: &[Orbit]) {
    let first_tree = find_all_ancestors(first, orbits);
    let second_tree = find_all_ancestors(second, orbits);
    let mut min_jump  = usize::MAX;
    for (index_first, first) in first_tree.iter().enumerate() {
        for (index_second, second) in second_tree.iter().enumerate() {
            if first == second {
                if index_first + index_second < min_jump {
                    min_jump = index_first + index_second;
                }
            }
        }
    }
    dbg!(min_jump);
}

fn find_all_ancestors<'a>(orb: &str, orbits: &'a [Orbit]) -> Vec<&'a String> {
    let mut vec = Vec::new();
    for orbit in orbits {
        if orbit.name.as_str() != orb {
            continue;
        }
        let mut current_parent = &orbit.parent;
        vec.push(current_parent);
        loop {
            if current_parent.as_str() == "COM" {
                break;
            }
            current_parent = search_for_parent(&orbits, current_parent).unwrap();
            vec.push(current_parent);
        }
    }
    vec
}

#[derive(Debug, Clone, PartialEq)]
struct Orbit {
    name: String,
    parent: String
}