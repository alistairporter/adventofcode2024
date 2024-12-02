use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn check_line(line: Vec<i32>, debug: bool) -> bool {
    let mut line_unsafe = 0;
    let mut line_safe = 0;
    let mut line_neg_count = 0;
    let mut line_pos_count = 0;

    for (pos, e) in (&line).iter().enumerate() {
        if debug { println!("Element at position {}: {:?}", pos, e);}
        if pos != 0 {
            let diff = line[pos-1].abs_diff(*e);
            let signed_diff = line[pos-1] - e;

            if debug {
                println!("diff is {}",diff);
                println!("signed_diff is {}",signed_diff);
            }
            
            if signed_diff < 0 {
                line_neg_count += 1;
            }
            if signed_diff > 0 {
                line_pos_count += 1;
            }

            if diff > 0 {
                line_safe = 1;
            }
            if diff < 4 {
                line_safe = 1;
            }
            if diff > 3 {
                line_unsafe = 1;
            }
            if diff == 0 {
                line_unsafe = 1;
            }
        }
    }
    
    if debug {
        println!("line neg count: {:?}", line_neg_count);
        println!("line pos count: {:?}", line_pos_count);
    }
    
    if (line_neg_count == 0 || line_pos_count == 0 ) && (line_unsafe == 0 && line_safe == 1) {
        return true;
    }
    else {
        return false;
    }
}

fn main() {
    let debug: bool = true;
    let mut safe_count = 0;
        
    let lines = lines_from_file("./input.txt").expect("Could not load lines");
    for line in lines {        
        let split_line = line.split(" ");
        let mut parsed_line: Vec<i32> = vec![];
        for item in split_line {
            let mut int_item: i32  = item.parse().expect("here be dragons");
            parsed_line.push(int_item);
        }

        let dampening_check_line = parsed_line.clone();
        
        let result = check_line(parsed_line, debug);

        if result == true {
            if debug { println!("SAFE LINE!"); }
            safe_count += 1;
        }
        else {
            let mut dampened_success = 0; 
            for (pos, e) in (&dampening_check_line).iter().enumerate() {
                let mut modified_line: Vec<i32> = vec![];

                modified_line.clone_from(&dampening_check_line);
                modified_line.remove(pos);

                if debug {println!("{:?}",modified_line);}

                if check_line(modified_line, debug) {
                    dampened_success += 1 
                }
                
            }
            if dampened_success > 0 {
                if debug { println!("DAMPENED SAFE LINE! +++++++++++++++++++++"); }
                safe_count += 1;
            } else {
                if debug { println!("UNSAFE LINE!-----------------------------"); }
            }
        }
        
        if debug {
            println!("safe line count: {:?}", &safe_count);
            println!("\n\n");
        }
   
    }

    println!("FINAL SAFE COUNT: {:?}", safe_count)
}
