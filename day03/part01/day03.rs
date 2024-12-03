use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use regex::Regex;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn parse_line(line: String, debug: bool) -> i32 {
    let mut line_total = 0;
    let re = Regex::new(r"mul\((?<digit1>[0-9]{1,3}),(?<digit2>[0-9]{1,3})\)").unwrap();

    let digits: Vec<(&str, &str)> = re.captures_iter(line.as_str()).map(|caps| {
    
        let digit1 = caps.name("digit1").unwrap().as_str();
        let digit2 = caps.name("digit2").unwrap().as_str();
        (digit1, digit2)
    }).collect();

    if debug {println!("{:?}", &digits);}
    
    for pair in digits {
        let pair1_int: i32 = pair.0.parse().expect("FAIL");
        let pair2_int: i32 = pair.1.parse().expect("FAIL");
        let value = pair1_int * pair2_int;
        line_total += value;
    }
    return line_total
//  return 161;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test_lines = lines_from_file("./test.txt").expect("Could not load lines");
    
        let result = parse_line(test_lines[0].clone(), false);
        assert_eq!(result, 161);
    }
}

fn main() {
    let debug: bool = true;
    let mut mult_total = 0;
    let lines = lines_from_file("./input.txt").expect("Could not load lines");
    for line in lines {
        let result = parse_line(line, debug);
        mult_total += result;
    }
    println!("FINAL MULTIPLICATION RESULT: {:?}", mult_total)
}
