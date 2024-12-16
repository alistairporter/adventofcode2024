struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn extract_values(line: String) -> (i64, i64) {
    let (_, right_side) = line.split_once(": ").unwrap();
    let (x_string, y_string) = right_side.split_once(", ").unwrap();
    let x: i64 = x_string[2..].parse().unwrap();
    let y: i64 = y_string[2..].parse().unwrap();
    return (x, y);
}

pub fn main(data: Vec<String>) -> i64 {
    let mut lines = data.into_iter();
    
    let mut machines: Vec<Machine> = vec![];
    let mut tokens = 0;
    
    loop {
        let a = extract_values(lines.next().unwrap());
        let b = extract_values(lines.next().unwrap());
        let prize = extract_values(lines.next().unwrap());

        machines.push(Machine { a_x: a.0, a_y: a.1, b_x: b.0, b_y: b.1, prize_x: prize.0, prize_y: prize.1 });

        if lines.next() == None { break; }
    }
    
    for machine in machines {
        // use cramer's rule to solve with 2 unknowns:    
        // find value for a
        let a: f64 = (machine.prize_x * machine.b_y - machine.prize_y * machine.b_x) as f64 / (machine.a_x * machine.b_y - machine.a_y * machine.b_x) as f64;

        // use a to find b by plugging into first equation
        let b: f64 = (machine.prize_x as f64 - a * machine.a_x as f64) / machine.b_x as f64;

        if a.fract() != 0.0 || b.fract() != 0.0 { 
            // not a whole number, there is no way to win on this machine
            continue; 
        }

        // 3 tokens to push a, 1 token for b 
        tokens += a as i64 * 3 + b as i64;
    }

    return tokens;
}
