// use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;


fn find_location(steps: &Vec<Vec<&str>>) -> (i16, i16) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut face = "N";
    //let mut seen: HashSet<(i16, i16)> = HashSet::new();
    //let mut seen_twice: (i16, i16) = (0, 0);
    for step in steps {
        let step_int = step[1].parse().unwrap();
        match (face, step[0]) {
            ("S", "L") | ("N", "R") => {x += step_int; face = "W"},

            ("W", "L") | ("E", "R") => {y += step_int; face = "N"},

            ("S", "R") | ("N", "L") => {x -= step_int; face = "E"},

            ("E", "L") | ("W", "R") => {y -= step_int; face = "S"},
            _ => {},
        };

    };
    (x.abs(), y.abs())

}

fn main() {
    let path = Path::new("data/input.txt");
    let mut file = File::open(&path).expect("File read error");
    let mut s = String::new();

    file.read_to_string(&mut s).unwrap();
    let steps: Vec<Vec<&str>> = s.split(',').map(|r| {
        r.trim().splitn(3, "").filter(|&s| s != "").collect()
    }).collect();
    let (x, y) = find_location(&steps);
    println!("Answer 1: {} + {} = {}", x, y, x + y);

}
