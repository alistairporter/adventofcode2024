use std::collections::BinaryHeap;

pub fn part01(line: &str) -> i64 {
    let mut csum: i64 = 0;

    let mut disk: Vec<i64> = vec![];
    let mut heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut id: i32 = 0;
    let chars: Vec<_> = line.chars().collect();
    for index in (0..line.len()).step_by(2) {
        for _ in 0..chars[index].to_digit(10).unwrap() {
            disk.push(id.into());
            heap.push(id.into());
        }
        if chars.get(index + 1) != None {
            for _ in 0..chars[index + 1].to_digit(10).unwrap() {
                disk.push(-1);
            }
        }
        id += 1;
    }

    for index in 0..disk.len() {
        if disk[index] == -1 {
            let number = match heap.pop() {
                Some(number) => number,
                None => break
            };
            disk[index] = number;
            let position = disk.iter().rposition(|it| *it == number).unwrap();
            disk[position] = -1;
        }
    }

    for (index, number) in disk.iter().enumerate() {
        if *number != -1 {
            csum += index as i64 * *number;
        }
    }

    return csum;
}
