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
    println!("{}", not_triangles_count);

}


#[test]
fn test_get_button_coords() {
    let start: (u8, u8) = (1, 1);
    assert_eq!(get_button_coords(&start, "ULL"), (0, 0)); // 1
    assert_eq!(get_button_coords(&(0, 0), "RRDDD"), (2, 2)); // 9
    assert_eq!(get_button_coords(&(2, 2), "LURDL"), (1, 2)); // 8
    assert_eq!(get_button_coords(&(1, 2), "UUUUD"), (1, 1)); // 5


}
