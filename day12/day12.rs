mod part01;
mod part02;
use std::fs::File;
use std::io::{BufRead, BufReader};

// HELPER FUNCTIONS

pub fn parse_input(filename: &str ) -> Vec<Vec<char>> {
    let file = BufReader::new(File::open(filename).expect("FAIL"));
    let mut input2d: Vec<Vec<char>> = vec![];
    
    for line in file.lines() {
        let mut row: Vec<char> = vec![];
        for char in line.unwrap().chars() {
            row.push(char);
        }
        input2d.push(row);
    }
    return input2d;
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = parse_input("./test.txt");
        assert_eq!(part01::part01(data), 1930);
    }
    #[test]
    fn test_part02() {
        let data = parse_input("./test.txt");
        assert_eq!(part02::part02(data), 1206);
    }
}

// ENTRYPOINT
fn main() {
    let data = parse_input("input.txt");

    println!("Part 1: {}", part01::part01(data.clone()));
    println!("Part 2: {}", part02::part02(data));
}
