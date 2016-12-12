use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let path = Path::new("data/input.txt");
    let mut file = File::open(&path).expect("File read error");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut index_sum: u32 = 0;


    for line in content.trim().split('\n') {
        let mut letters_count: HashMap<char, u8> = HashMap::new();
        let parts: Vec<&str> = line.rsplitn(2, '-').collect();

        for ch in parts[1].chars().filter(|&c| c != '-') {
            let counter = letters_count.entry(ch).or_insert(0);
            *counter += 1;
        }
        let mut letters: Vec<(u8, char)> =
            letters_count.iter().map(|(&key, &val)| (val, key)).collect();
        letters.sort_by_key(|&k| (-(k.0 as i32), k.1));
        let info: Vec<&str> = parts[0].split(|c| (c == '[') | (c == ']')).collect();
        if info[1] == letters.iter().take(5).map(|&(_, c)| c).collect::<String>() {
            index_sum += info[0].parse().unwrap();
        }

    }
    println!("Answer 1: {}", index_sum)

}
