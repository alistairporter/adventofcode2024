use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn to_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn rotate_90_deg_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}


fn in_bounds(position: (i32, i32), height: usize, width: usize) -> bool {
    position.0 >= 0 && position.0 < width as i32 && position.1 >= 0 && position.1 < height as i32
}

pub fn part1(char_matrix: Vec<Vec<char>>) -> i32 {
    let mut set = HashSet::new();
    let height = char_matrix.len();
    let width = char_matrix.first().unwrap().len();
    let mut guard_position = (0, 0);
    let mut guard_direction = Direction::Up;
    let mut guard_starting_position = guard_position;
    let mut result = 0;

    for x in 0..width {
        for y in 0..height {
            if char_matrix[y][x] == '^' {
                guard_position = (x as i32, y as i32);
                guard_starting_position = guard_position;
            }
        }
    }

    while in_bounds(guard_position, height, width) {
        set.insert((guard_position.0, guard_position.1, guard_direction));
        let direction = guard_direction.to_tuple();
        let new_position = (
            guard_position.0 + direction.0,
            guard_position.1 + direction.1,
        );

        if !in_bounds(new_position, height, width) {
            break;
        }
        
        if char_matrix[new_position.1 as usize][new_position.0 as usize] == '#' {
            guard_direction = guard_direction.rotate_90_deg_right();
        } else {
            guard_position = new_position
        }
    }
    result = set
        .iter()
        .map(|&(a, b, _)| (a, b))
        .collect::<HashSet<_>>()
        .len();

    result.try_into().unwrap()
}
