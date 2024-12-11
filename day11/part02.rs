use std::collections::HashMap;

pub fn part02(nums: Vec<usize>) -> usize {
    let iters: usize = 75;
    let mut counts = HashMap::<usize, usize>::new();
    nums.into_iter()
        .for_each(|x| *counts.entry(x).or_default() += 1);

    for _ in 0..iters {
        let mut next = HashMap::new();
        for (stone, count) in counts {
            if stone == 0 {
                *next.entry(1).or_default() += count;
            } else if let Some((a, b)) = split_digits(stone) {
                *next.entry(a).or_default() += count;
                *next.entry(b).or_default() += count;
            } else {
                *next.entry(stone * 2024).or_default() += count;
            }
        }
        counts = next;
    }

    counts.values().sum::<usize>().into()
}

fn split_digits(num: usize) -> Option<(usize, usize)> {
    let digits = num.ilog10() + 1;
    let pow = 10_usize.pow(digits / 2);
    (digits & 1 == 0).then(|| (num / pow, num % pow))
}
