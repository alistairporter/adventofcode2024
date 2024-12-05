use std::cmp::Ordering;

pub fn task2(ordering_rules: Vec<(i32, i32)>,mut updates: Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;

    for update in &mut updates {
        let mut order_incorrect = false;

        for order in &ordering_rules {
            if let Some(first) = update.iter().position(|&item| item == order.0) {
                if let Some(second) = update.iter().position(|&item| item == order.1) {
                    if first > second {
                        order_incorrect = true;
                        break;
                    }
                }
            }
        }

        if order_incorrect {
            update.sort_unstable_by(|&a, &b| fix_order(a, b, &ordering_rules));
            answer += update[update.len() / 2];
        }
    }

    return answer;
}

fn fix_order(a: i32, b: i32, ordering_rules: &Vec<(i32, i32)> ) -> Ordering {
    if ordering_rules.contains(&(a, b)) {
        return Ordering::Less;
    }
    if ordering_rules.contains(&(b, a)) {
        return Ordering::Greater;
    }
    Ordering::Equal
}
