mod part01;
mod part02;

use part01::count_xmas;
use part02::count_cross_mas;

use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let char_matrix = get_input("./test.txt");
        assert_eq!(count_xmas(&char_matrix), 18);
    }
    
    #[test]
    fn test_part02() {
        let char_matrix = get_input("./test.txt");
        assert_eq!(count_cross_mas(&char_matrix), 9);
    }
}

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

// ENTRYPOINT

fn main() {
     let char_matrix = get_input("./input.txt");
 
     println!("RESULT 1: {}", count_xmas(&char_matrix));
     println!("RESULT 2: {}", count_cross_mas(&char_matrix));
}
