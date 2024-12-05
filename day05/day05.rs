mod part01;
use part01::task1;

mod part02;
use part02::task2;

use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::fs::File;

// HELPER FUNCTIONS

fn parse_input(filename: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let input = read_lines_from_file(filename).expect("FAIL");

    let mut ordering_rules: Vec<(i32, i32)> = vec![];
    let mut updates = vec![];

    let mut read_orders = false;
    
    for line in input {
        if line.is_empty(){
            read_orders = true;
            continue;
        }

        if read_orders {
            let mut line_updates: Vec<i32> = vec![];
            let split_updates = line.split(",");

            for item in split_updates {
                let int_update_item: i32  = item.parse().expect("here be dragons");
                line_updates.push(int_update_item);
            }
            
            updates.push(line_updates);           
        } else {
            let split_rules = line.split("|");
            let mut line_rules: Vec<i32> = vec![];

            for item in split_rules {
                let int_rule_item: i32 = item.parse().expect("here be dragons");
                line_rules.push(int_rule_item);
            }
            
            ordering_rules.push((line_rules[0], line_rules[1]));
        }
    }

    return (ordering_rules, updates);

    
}

fn read_lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let (ordering_rules, updates) = parse_input("./test.txt");
        assert_eq!(task1(ordering_rules, updates), 143);
    }
    #[test]
    fn test_part02() {
        let (ordering_rules, updates) = parse_input("./test.txt");
        assert_eq!(task2(ordering_rules, updates), 123);
    }
}

// ENTRYPOINT
fn main() {
    let (ordering_rules, updates) = parse_input("./input.txt");

     println!("RESULT 1: {}", task1(ordering_rules.clone(), updates.clone()));
     println!("RESULT 2: {}", task2(ordering_rules, updates));
}
