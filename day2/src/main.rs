use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type ButtonLine = (u8, u8, u8);


const BUTTONS: [[u8; 3]; 3] = [[1, 2, 3],
                               [4, 5, 6],
                               [7, 8, 9]];


fn get_button_coords(start_on: &(u8, u8), steps: &str) -> (u8, u8) {
    let mut x = start_on.0;
    let mut y = start_on.1;
    for direction in steps.chars() {
        match direction {
            'U' if y > 0 => y -= 1,
            'D' if y < 2 => y += 1,
            'L' if x > 0 => x -= 1,
            'R' if x < 2 => x += 1,
            _ => {},
        }
    }
    (x, y)
}


fn main() {
    let path = Path::new("data/input.txt");
    let mut file = File::open(&path).expect("File read error");
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let mut buttons_list: Vec<u8> = vec!();
    let start_from: (u8, u8) = (1, 1);

    for steps in s.trim().lines() {
        let start_from = get_button_coords(&start_from, steps);
        buttons_list.push(BUTTONS[start_from.1 as usize][start_from.0 as usize]);
    }

    println!("Buttons: {:?}", buttons_list);

}


#[test]
fn test_get_button_coords() {
    let start: (u8, u8) = (1, 1);
    assert_eq!(get_button_coords(&start, "ULL"), (0, 0)); // 1
    assert_eq!(get_button_coords(&(0, 0), "RRDDD"), (2, 2)); // 9
    assert_eq!(get_button_coords(&(2, 2), "LURDL"), (1, 2)); // 8
    assert_eq!(get_button_coords(&(1, 2), "UUUUD"), (1, 1)); // 5


}
