mod part1;
mod part2;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

// HELPER FUNCTIONS

fn read_input<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).expect("Failed to read file");
    io::BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
}

fn get_input(filename: &str) -> Vec<Vec<char>> {
    let char_matrix: Vec<Vec<char>> = read_input(filename)
        .map(|line| line.chars().collect())
        .collect();
    
    return char_matrix;
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let char_matrix = get_input("./test.txt");
        assert_eq!(part1::part1(char_matrix), 41);
    }
    #[test]
    fn test_part02() {
        let char_matrix = get_input("./test.txt");
        assert_eq!(part2::part2(char_matrix), 6);
    }
}

fn main() {
    let char_matrix = get_input("./input.txt");

    println!("RESULT 1: {}", part1::part1(char_matrix.clone()));
    println!("RESULT 2: {}", part2::part2(char_matrix.clone()));
}
