// use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;


fn find_location(steps: &Vec<Vec<&str>>) -> (i16, i16, i16, i16) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut face = "N";
    let mut seen: HashSet<(i16, i16)> = HashSet::new();
    let mut seen_twice: (i16, i16) = (0, 0);
    for (index, step) in steps.iter().enumerate() {
        {
            let direction = match index % 2 {
                0 => &mut x,
                _ => &mut y,
            };
            let step_int = step[1].parse().unwrap();
            match (face, step[0]) {
                ("N", "R") => {*direction += step_int; face = "W"},
                ("N", "L") => {*direction -= step_int; face = "E"},

                ("W", "L") => {*direction += step_int; face = "N"},
                ("W", "R") => {*direction -= step_int; face = "S"},

                ("S", "L") => {*direction += step_int; face = "W"},
                ("S", "R") => {*direction -= step_int; face = "E"},

                ("E", "R") => {*direction += step_int; face = "N"},
                ("E", "L") => {*direction -= step_int; face = "S"},
                _ => {},
            };
        }
        if seen_twice == (0, 0) && seen.contains(&(x, y)) {
            seen_twice = (x, y);
            println!("{} {}", x, y);
        } else {
            seen.insert((x, y));
        }

    };
    (x.abs(), y.abs(), seen_twice.0.abs(), seen_twice.1.abs())

}

fn main() {
    let path = Path::new("data/input.txt");
    let mut file = File::open(&path).expect("File read error");
    let mut s = String::new();

    file.read_to_string(&mut s).unwrap();
    let steps: Vec<Vec<&str>> = s.split(',').map(|r| {
        r.trim().splitn(3, "").filter(|&s| s != "").collect()
    }).collect();
    let (x, y, x1, y1) = find_location(&steps);
    println!("Answer 1: {} + {} = {}", x, y, x + y);
    println!("Answer 2: {} + {} = {}", x1, y1, x1 + y1);

}
