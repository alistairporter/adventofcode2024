mod part01;
mod part02;
use std::fs::File;
use std::io::{BufRead, BufReader};

// HELPER FUNCTIONS

pub fn parse_input(filename: &str ) -> Vec<String> {
    let file = BufReader::new(File::open(filename).expect("FAIL"));

    let mut lines: Vec<String> = vec![];

    for line in file.lines() {
        lines.push(line.unwrap()); 
    }
    return lines;
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = parse_input("./test.txt");
        assert_eq!(part01::main(data), 10092);
    }
    #[test]
    fn test_part02() {
        let data = parse_input("./test.txt");
        assert_eq!(part02::main(data), 9021);
    }
}

// ENTRYPOINT
fn main() {
    let data = parse_input("input.txt");

    println!("Part 1: {}", part01::main(data.clone()));
    println!("Part 2: {}", part02::main(data));
}
