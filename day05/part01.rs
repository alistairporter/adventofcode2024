pub fn task1(ordering_rules: Vec<(i32, i32)>, updates: Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;

    for update in updates {
        let mut order_correct = true;

        for order in &ordering_rules {
            if let Some(first) = update.iter().position(|&item| item == order.0) {
                if let Some(second) = update.iter().position(|&item| item == order.1) {
                    if first > second {
                        order_correct = false;
                        break;
                    }
                }
            }
        }

        if order_correct {
             answer += update[update.len() / 2];
        }
    }

    return answer;
}
