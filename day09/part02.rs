use std::collections::BinaryHeap;

pub fn part02(line: &str) -> i64 {
    let mut csum: i64 = 0;

    let mut disk: Vec<(i64, i64)> = vec![];
    let mut heap: BinaryHeap<(i64, i64)> = BinaryHeap::new();
    let mut id: i32 = 0;
    let chars: Vec<_> = line.chars().collect();
    for index in (0..line.len()).step_by(2) {
        disk.push((id.into(), chars[index].to_digit(10).unwrap().into()));
        heap.push((id.into(), chars[index].to_digit(10).unwrap().into()));
        if chars.get(index + 1) != None {
            disk.push((-1, chars[index + 1].to_digit(10).unwrap().into()));
        }
        id += 1;
    }

    loop {
        if heap.peek() == None { break; }

        let (id, length) = heap.pop().unwrap();
        if let Some(index) = disk.iter().position(|it| it.0 == -1 && it.1 >= length) {
           let space = disk[index].1;
            disk.splice(index..index + 1, [(id, length), (-1, space - length)]);
            let position = disk.iter().rposition(|it| it.0 == id).unwrap();
            disk.splice(position..position + 1, [(-1, length)]);
        }
    }

    let mut index: i64 = 0;
    for segment in disk.iter() {
        if segment.0 == -1 { 
            index += segment.1;
        }
        else {
            for _ in 0..segment.1 {
                csum += segment.0 * index;
                index += 1;
            }
        }
    }

    return csum;
}
