mod part01;
mod part02;
use std::fs::File;
use std::io::{BufRead, BufReader};

//use part01;

// HELPER FUNCTIONS

pub fn parse_input(filename: &str) -> (Vec<Vec<i32>>, Vec<(usize, usize)>, usize, usize) {
    let file = BufReader::new(File::open(filename).expect("FAIL"));
 
    let mut grid: Vec<Vec<i32>> = vec![];
    let mut trailheads: Vec<(usize, usize)> = vec![];
    
    for (y, line) in file.lines().enumerate() {
        let mut row: Vec<i32> = vec![];
        for (x, character) in line.unwrap().chars().enumerate() {
            row.push(character.to_digit(10).unwrap() as i32);
            if character == '0' {
               trailheads.push((x, y)); 
            }
        }
        grid.push(row);
    }

    let height = grid.len();
    let width = grid[0].len();

    return (grid, trailheads, height, width)
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let (grid, trailheads, height, width) = parse_input("./test.txt");
        assert_eq!(part01::part01(&grid, &trailheads, width, height), 36);
    }
    #[test]
    fn test_part02() {
        let (grid, trailheads, height, width) = parse_input("./test.txt");
        assert_eq!(part02::part02(&grid, &trailheads, width, height), 81);
    }
}

// ENTRYPOINT
fn main() {
    let (grid, trailheads, height, width) = parse_input("input.txt");

    println!("Part 1: {}", part01::part01(&grid, &trailheads, width, height));
    println!("Part 2: {}", part02::part02(&grid, &trailheads, width, height));
}
