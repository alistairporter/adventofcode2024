mod part01;
mod part02;
use std::fs;
use std::path::Path;

// HELPER FUNCTIONS

pub fn parse_input(filename: &str) -> Vec<usize> {
    let input = fs::read_to_string(Path::new(filename)).expect("FAIL");

    return input.split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = parse_input("./test.txt");
        assert_eq!(part01::part01(data), 55312);
    }
    #[test]
    fn test_part02() {
        let data = parse_input("./test.txt");
        assert_eq!(part02::part02(data), 65601038650482);
    }
}

// ENTRYPOINT
fn main() {
    let data = parse_input("input.txt");

    println!("Part 1: {}", part01::part01(data.clone()));
    println!("Part 2: {}", part02::part02(data));
}
