mod part01;
mod part02;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
//use part01;

// HELPER FUNCTIONS
fn read_lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}


fn parse_input(filename: &str) -> Vec<(usize, Vec<usize>)> {
    let input = read_lines_from_file(filename).expect("FAIL");
    
    let mut data = vec![];
     
    for line in input {
        let mut halves = line.split(": ");
        let testvalue: usize = halves.next().unwrap().parse::<usize>().unwrap();
        let numbers: Vec<usize> = halves.next().unwrap().split(" ").into_iter().map(|number| number.parse().unwrap()).collect();

        data.push((testvalue, numbers));

    }

    return data
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = parse_input("./test.txt");
        assert_eq!(part01::part01(data), 3749);
    }
    #[test]
    fn test_part02() {
        let data = parse_input("./test.txt");
        assert_eq!(part02::part02(data), 11387);
    }
}

// ENTRYPOINT
fn main() {
    let data = parse_input("input.txt");

    println!("Part 1: {}", part01::part01(data.clone()));
    println!("Part 2: {}", part02::part02(data.clone()));
}
