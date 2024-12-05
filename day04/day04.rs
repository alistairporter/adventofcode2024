mod part01;

use part01::count_xmas;

use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let char_matrix: Vec<Vec<char>> = read_input("./test.txt")
        .map(|line| line.chars().collect())
        .collect();
        assert_eq!(count_xmas(&char_matrix), 18);
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

// ENTRYPOINT

fn main() {
    let char_matrix: Vec<Vec<char>> = read_input("./input.txt")
        .map(|line| line.chars().collect())
        .collect();
     
     println!("RESULT 1: {}", count_xmas(&char_matrix));
}
