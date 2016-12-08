use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//fn is_triangle(sides: [u16, u16, u16]) -> bool {
//    sides.sort();
//
//
//
//}

fn main() {
    let path = Path::new("data/input.txt");
    let mut file = File::open(&path).expect("File read error");
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let not_triangles_count = s.trim()
                               .lines()
                               .map(|line| {
                                    let  mut sides: Vec<u16> = line.split_whitespace().map(|l| l.parse().unwrap()).collect::<Vec<u16>>();
                                    sides.sort();
                                    sides[0] + sides[1] > sides[2]
                               })
                               .filter(|&r| r == true)
                               .count();
    println!("Part 1: {}", not_triangles_count);
    let mut break_occurs: u8 = 0;
    let not_triangles_count: u16 = s.trim()
                               .split(|c: char| if c != '\n' { false }
                                                else if break_occurs == 2 {break_occurs = 0; true} 
                                                else {break_occurs += 1; false})
                               .map(|three_lines| {
                                    let vecs: Vec<Vec<u16>> = three_lines.lines()
                                                                         .map(|line| line.split_whitespace()
                                                                                         .map(|l| l.parse()
                                                                                                   .unwrap())
                                                                                         .collect::<Vec<u16>>())
                                                                         .collect();
                                    let mut good_count: u16 = 0;
                                    for x in 0..3 {
                                        let mut l: Vec<u16> = (0..3).map(|y| vecs[y][x]).collect();
                                        l.sort();
                                        if l[0] + l[1] > l[2] {
                                            good_count += 1;
                                        };
                                        
                                    };
                                    good_count
                               })
                               .sum();
    println!("Part 2: {}", not_triangles_count);

}

