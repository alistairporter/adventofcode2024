mod part01;
mod part02;
use std::fs::File;
use std::io::{BufRead, BufReader};

//use part01;

// HELPER FUNCTIONS

pub fn parse_input(filename: &str) -> String {
    let file = BufReader::new(File::open(filename).expect("FAIL"));
    
    let mut data: String = "".to_owned();
    
    for line in file.lines() {
        data.push_str(&line.unwrap()); 
    }
    return data;
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = parse_input("./test.txt");
        assert_eq!(part01::part01(&data), 1928);
    }
    #[test]
    fn test_part02() {
        let data = parse_input("./test.txt");
        assert_eq!(part02::part02(&data), 2858);
    }
}

// ENTRYPOINT
fn main() {
    let data = parse_input("input.txt");

    println!("Part 1: {}", part01::part01(&data));
    println!("Part 2: {}", part02::part02(&data));
}
