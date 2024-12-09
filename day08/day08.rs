mod part01;
mod part02;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
//use part01;

// HELPER FUNCTIONS
fn parse_input(filename: &str) -> (HashMap<char, Vec<(i32, i32)>>, usize, usize) {
    let file = File::open(filename).expect("FAIL");
    let input = BufReader::new(file);

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    
    for (y, line) in input.lines().enumerate() {
        let line = line.unwrap();
        height += 1;
        if y == 0 {
            width = line.len();
        }
        for (x, character) in line.chars().enumerate() {
            if character != '.' && antennas.contains_key(&character) {
                antennas.get_mut(&character).unwrap().push((x as i32, y as i32));
            }
            else {
               antennas.insert(character, vec![(x as i32, y as i32)]); 
            }
        }
    }

    return (antennas, width, height);
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let (antennas, width, height) = parse_input("./test.txt");
        assert_eq!(part01::part01(&antennas, width, height), 14);
    }
    #[test]
    fn test_part02() {
        let (antennas, width, height) = parse_input("./test.txt");
        assert_eq!(part02::part02(&antennas, width, height), 34);
    }
}

// ENTRYPOINT
fn main() {
    let (antennas, width, height) = parse_input("input.txt");

    println!("Part 1: {}", part01::part01(&antennas, width.clone(), height.clone()));
    println!("Part 2: {}", part02::part02(&antennas, width.clone(), height.clone()));
}
