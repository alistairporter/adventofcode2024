use itertools::Itertools;
use std::iter;

fn valid_equation(testvalue: usize, numbers: Vec<usize>) -> bool {
    let operators = vec!['+', '*', '|'];

    let permutations = iter::repeat_n(operators.into_iter(), numbers.len() - 1).multi_cartesian_product();
    for permutation in permutations {
        let mut sum = numbers[0];
        for i in 0..permutation.len() {
            sum = match permutation[i] {
                '+' => sum + numbers[i + 1],
                '*' => sum * numbers[i + 1],
                '|' => format!("{}{}", sum, numbers[i + 1]).parse().unwrap(),
                _ => sum
            };
            if sum > testvalue {
                break;
            }
        }
        if sum == testvalue { 
            return true; 
        } 
    }
    return false;
}

pub fn part02(data: Vec<(usize, Vec<usize>)>) -> usize {
    let mut result = 0;
    
    for line in data {
        let testvalue = line.0;
        let numbers = line.1;
        
        if valid_equation(testvalue.clone(), numbers.clone()) {
            result += testvalue;
        }
    }
    return result
}
