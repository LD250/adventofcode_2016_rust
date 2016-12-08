use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

type ButtonLine = (u8, u8, u8);


const BUTTONS: [[char; 3]; 3] = [['1', '2', '3'],
                                 ['4', '5', '6'],
                                 ['7', '8', '9']];

const EXTENDED_BUTTONS: [[char; 5]; 5] = [['0', '0', '1', '0', '0'],
                                          ['0', '2', '3', '4', '0'],
                                          ['5', '6', '7', '8', '9'],
                                          ['0', 'A', 'B', 'C', '0'],
                                          ['0', '0', 'D', '0', '0']];

fn get_button_coords(start_on: &(usize, usize), steps: &str) -> (usize, usize) {
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


fn get_extended_button_coords(start_on: &(usize, usize), steps: &str) -> (usize, usize) {
    let mut x = start_on.0;
    let mut y = start_on.1;
    for direction in steps.chars() {
        match direction {
            'U' if y > 0 && EXTENDED_BUTTONS[y - 1][x] != '0' => y -= 1,
            'D' if y < 4 && EXTENDED_BUTTONS[y + 1][x] != '0' => y += 1,
            'L' if x > 0 && EXTENDED_BUTTONS[y][x - 1] != '0' => x -= 1,
            'R' if x < 4 && EXTENDED_BUTTONS[y][x + 1] != '0' => x += 1,
            _ => {},
        }
    }
    (x, y)
}

macro_rules! print_buttons {
    ( $( [$func_name:ident, $steps_instr:expr, $buttons:expr]; )+ ) => {
        {
            $(
                let mut buttons_list: Vec<char> = vec!();
                let start_from: (usize, usize) = (1, 1);

                for steps in $steps_instr.trim().lines() {
                    let start_from = $func_name(&start_from, steps);
                    buttons_list.push($buttons[start_from.1][start_from.0]);
                }

                println!("Buttons: {:?}", buttons_list);

            )*
        }
    };
}

fn main() {
    let path = Path::new("data/input.txt");
    let mut file = File::open(&path).expect("File read error");
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    print_buttons!([get_button_coords, s, BUTTONS];
                   [get_extended_button_coords, s, EXTENDED_BUTTONS];);

}


#[test]
fn test_get_button_coords() {
    let start: (u8, u8) = (1, 1);
    assert_eq!(get_button_coords(&start, "ULL"), (0, 0)); // 1
    assert_eq!(get_button_coords(&(0, 0), "RRDDD"), (2, 2)); // 9
    assert_eq!(get_button_coords(&(2, 2), "LURDL"), (1, 2)); // 8
    assert_eq!(get_button_coords(&(1, 2), "UUUUD"), (1, 1)); // 5


}
