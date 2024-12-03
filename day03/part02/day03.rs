use std::fs;
use std::error::Error;
use regex::Regex;

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = read_input("./test.txt").expect("FAIL");
        let result = solve(input, false);
        assert_eq!(result, 48);
    }
}

// HELPER FUNCTIONS
fn read_input(file: &str) -> Result<String, Box<dyn Error>> {
    let message: String = fs::read_to_string(file)?;
    Ok(message)
}

fn solve(input: String, _debug: bool) -> i32 {
    let re = Regex::new(r"(?P<donot>don't)|(?P<do>do)|(?P<operation>mul[(](?P<digit1>\d+),(?P<digit2>\d+)[)])").unwrap();    
    let mut line_total = 0;
    let mut flag_do = true;

    for m in re.captures_iter(&input) {
        if m.name("do").is_some() {
            
            flag_do = true;
        } else if m.name("donot").is_some() {
            
            flag_do = false;
        } else if flag_do && m.name("operation").is_some() {
            
            let digit1: i32 = m.name("digit1").unwrap().as_str().parse().unwrap();
            let digit2: i32 = m.name("digit2").unwrap().as_str().parse().unwrap();
            line_total += digit1 * digit2;
        }
    }
    return line_total
}

// MAIN SPAGHETTI FUNCTION
fn main() -> Result<(), Box<dyn Error>> {
    let debug: bool = false;
    let input = read_input("./input.txt")?;
    let answer = solve(input, debug);
    println!("FINAL MULTIPLICATION RESULT: {:?}", answer);
    Ok(())
}
