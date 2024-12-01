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
    let mut list1 = vec![];
    let mut list2 = vec![];
    let mut total_diff = 0;
    for line in lines {        
        // println!("{:?}", line);
        let splitline: Vec<&str> = line.split("   ").collect();
        let var0: i32 = splitline[0].parse().expect("Failed to parse string to integer");
        let var1: i32 = splitline[1].parse().expect("Failed to parse string to integer");
        list1.push(var0);
        list2.push(var1);
    }
    list1.sort();
    list2.sort();
//    println!("list1 contents:");
//    for item in list1 {
//        println!("{:?}", item);
//    }
//    println!("list2 contents:");
//    for item in list2 {
//        println!("{:?}", item);
//    }
    for (pos, e) in (&list1).iter().enumerate() {
        println!("Element at position {}: {:?}", pos, e);
        let diff = list2[pos].abs_diff(*e);
        total_diff = total_diff + diff;
    }
    println!("{}",total_diff);
}
