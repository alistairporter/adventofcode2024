use std::collections::HashMap;

pub fn main(data: Vec<String>) -> i32 {
    let mut lines = data.into_iter();
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();

    let mut y: i32 = 0;
    let mut start: (i32, i32) = (0, 0);

    // read in the grid
    loop {
        let line = lines.next().expect("fail");
        if line.is_empty() { break; }
       
        for (x, character) in line.chars().enumerate() {
            match character {
                '@' => {
                    grid.insert((x as i32 * 2, y), '@');
                    grid.insert((x as i32 * 2 + 1, y), '.');
                    start = (x as i32 * 2, y);
                },
                'O' => {
                    grid.insert((x as i32 * 2, y), '[');
                    grid.insert((x as i32 * 2 + 1, y), ']');
                },
                _ => {
                    grid.insert((x as i32 * 2, y), character);
                    grid.insert((x as i32 * 2 + 1, y), character);
                }
            }
        }
        y += 1;
    } 

    // read in the moves
    let mut moves: Vec<char> = vec![];
    while let Some(line) = lines.next() {
        for character in line.chars() {
            moves.push(character);
        }
    }

    process(start, &mut grid, &moves); 
    
    // sum GPS coordinates
    let mut sum = 0;
    for (position, value) in grid.iter() {
        if *value == '[' {
           sum += 100 * position.1 + position.0; 
        }
    }
    return sum;
}

fn process(start: (i32, i32), grid: &mut HashMap<(i32, i32), char>, moves: &Vec<char>) {
    let mut position = start;
    for direction in moves {
        if check_space(position, *direction, grid) {
            position = move_stack(position, *direction, grid);
        }
    }
}

fn check_space(position: (i32, i32), direction: char, grid: &mut HashMap<(i32, i32), char>) -> bool {
    let vector = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid move found!")
    };

    let mut current_position = position;

    loop {
        current_position = add_vector(current_position, vector);
        match grid.get(&current_position) {
            // space
            Some(character) if *character == '.' => { 
                return true;
            },
            
            // wall
            Some(character) if *character == '#' => { return false; },
            
            // left side of a box
            Some(character) if *character == '[' => {
                if direction == '^' || direction == 'v' {
                    if !check_space(add_vector(current_position, (1, 0)), direction, grid) {
                        return false;
                    }
                }
            }
            // right side of a box
            Some(character) if *character == ']' => {
                if direction == '^' || direction == 'v' {
                    if !check_space(add_vector(current_position, (-1, 0)), direction, grid) {
                        return false;
                    }
                }
            }
            
            // keep checking
            Some(_) => {},
            
            // left grid
            None => { return false; }
        }
    }
}

fn move_stack(position: (i32, i32), direction: char, grid: &mut HashMap<(i32, i32), char>) -> (i32, i32) {
    let vector = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid move found!")
    };

    let mut current_position = position;
    let mut temporary = *grid.get(&current_position).unwrap();

    let mut to_fill = '.';
    loop {
        grid.insert(current_position, to_fill);
        
        if temporary == '.' { break; }
        
        else if current_position != position && temporary == '[' && (direction == '^' || direction == 'v') {
            move_stack(add_vector(current_position, (1, 0)), direction, grid);
        }

        else if current_position != position && temporary == ']' && (direction == '^' || direction == 'v') {
            move_stack(add_vector(current_position, (-1, 0)), direction, grid);
        }

        to_fill = temporary;
        current_position = add_vector(current_position, vector);
        temporary = *grid.get(&current_position).unwrap();
    }
    return add_vector(position, vector);
}

fn add_vector(position: (i32, i32), vector: (i32, i32)) -> (i32, i32) {
    (position.0 + vector.0, position.1 + vector.1)
}
