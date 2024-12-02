use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let lines = lines_from_file("./input.txt").expect("Could not load lines");
    let mut safe_count = 0;
    for line in lines {        
        // println!("{:?}", line);
        let mut line_unsafe = 0;
        let mut line_safe = 0;
        let mut line_neg_count = 0;
        let mut line_pos_count = 0;
        let splitline = line.split(" ");
        let mut splitlineint: Vec<i32> = vec![];
        for item in splitline {
            let mut int_item: i32  = item.parse().expect("here be dragons");
            splitlineint.push(int_item);
        }
        for (pos, e) in (&splitlineint).iter().enumerate() {
            println!("Element at position {}: {:?}", pos, e);
            if pos != 0 {
                let diff = splitlineint[pos-1].abs_diff(*e);
                let signed_diff = splitlineint[pos-1] - e;
                if signed_diff < 0 {
                    line_neg_count += 1;
                }
                if signed_diff > 0 {
                    line_pos_count += 1;
                }
                println!("diff is {}",diff);
                println!("signed_diff is {}",signed_diff);
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
        println!("line neg count: {:?}", line_neg_count);
        println!("line pos count: {:?}", line_pos_count);        
        if (line_neg_count == 0 || line_pos_count == 0 ) && (line_unsafe == 0 && line_safe == 1) {
            println!("SAFE LINE!");
            safe_count += 1;
        }
        else {
            println!("UNSAFE LINE! --------------------------------------------------------------------------");
        }
        println!("safe line count: {:?}", &safe_count);
        println!("\n\n");
    }
}
