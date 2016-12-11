extern crate seahash;
extern crate num;

use seahash::SeaHasher;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::hash::BuildHasherDefault;
use std::collections::HashSet;
use num::iter::range_step;

type SeaHashSet<K> = HashSet<K, BuildHasherDefault<SeaHasher>>;


macro_rules! simple_range {
    ( $prev:expr, $cur:expr ) => {
        {
            if $prev == $cur {
                range_step($prev, $prev + 1, 1)
            } else {
                let direction = if $prev > $cur {-1} else {1};
                range_step($prev + direction, $cur + direction, direction)
            }
        }
    };
}


fn find_location(steps: &Vec<Vec<&str>>) -> (i16, i16, i16, i16) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut face = "N";
    let mut seen: SeaHashSet<(i16, i16)> = SeaHashSet::default();
    let mut seen_twice: (i16, i16) = (0, 0);
    for step in steps {
        let step_int = step[1].parse().unwrap();
        let x_prev = x;
        let y_prev = y;
        match (face, step[0]) {
            ("S", "L") | ("N", "R") => {x += step_int; face = "W"},

            ("W", "L") | ("E", "R") => {y += step_int; face = "N"},

            ("S", "R") | ("N", "L") => {x -= step_int; face = "E"},

            ("E", "L") | ("W", "R") => {y -= step_int; face = "S"},
            _ => {},
        };
        for xi in simple_range!(x_prev, x) {
            for yi in simple_range!(y_prev, y) {
                if (seen_twice == (0, 0)) & !seen.insert((xi, yi)) {
                    seen_twice = (xi, yi);
                }
            }
        }

    };
    (x.abs(), y.abs(), seen_twice.0, seen_twice.1)

}

fn main() {
    let path = Path::new("data/input.txt");
    let mut file = File::open(&path).expect("File read error");
    let mut s = String::new();

    file.read_to_string(&mut s).unwrap();
    let steps: Vec<Vec<&str>> = s.split(',').map(|r| {
        r.trim().splitn(3, "").filter(|&s| s != "").collect()
    }).collect();
    let (x, y, x2, y2) = find_location(&steps);
    println!("Answer 1: {} + {} = {}", x, y, x + y);
    println!("Answer 2: {} + {} = {}", x2, y2, x2 + y2);

}
