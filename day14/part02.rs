use std::usize::MAX;

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn main(data: Vec<String>) -> usize {
    let seconds = 10000;

    let mut robots: Vec<Robot> = vec![];
    let mut lowest: usize = MAX;
    let mut second_seen = 0;

    for line in data {
        // create each robot
        let (left_side, right_side) = line.split_once(" ").unwrap();
        let (x_position_string, y_position_string) = left_side[2..].split_once(",").unwrap();
        let (x_velocity_string, y_velocity_string) = right_side[2..].split_once(",").unwrap();
        let x_position: i32 = x_position_string.parse().unwrap();
        let y_position: i32 = y_position_string.parse().unwrap();
        let x_velocity: i32 = x_velocity_string.parse().unwrap();
        let y_velocity: i32 = y_velocity_string.parse().unwrap();
        robots.push(Robot { position: (x_position, y_position), velocity: (x_velocity, y_velocity) });        
    }
    
    for second in 1..seconds {
        for index in 0..robots.len() {
            move_robot(&mut robots[index]);
        }
        let safety = calculate_safety(&robots);
        if safety < lowest {
            lowest = safety;
            second_seen = second;
        }
    }

    return second_seen;
}

fn move_robot(robot: &mut Robot) {
    robot.position.0 = robot.position.0 + robot.velocity.0;
    robot.position.1 = robot.position.1 + robot.velocity.1;

    // handle wrap arounds
    if robot.position.0 >= WIDTH {
        robot.position.0 = robot.position.0 - WIDTH ;
    }
    else if robot.position.0 < 0 {
        robot.position.0 = WIDTH + robot.position.0;
    }
    if robot.position.1 >= HEIGHT {
        robot.position.1 = robot.position.1 - HEIGHT;
    }
    else if robot.position.1 < 0 {
        robot.position.1 = HEIGHT + robot.position.1;
    }
}

fn calculate_safety(robots: &Vec<Robot>) -> usize {
    let mut quadrants: [usize; 4] = [0, 0, 0, 0];
    let x_midline: i32 = HEIGHT / 2;
    let y_midline: i32 = WIDTH / 2;
    for robot in robots.iter() {
        // left side
        if robot.position.0 < y_midline {
            if robot.position.1 < x_midline {
                quadrants[0] += 1;
            }
            else if robot.position.1 > x_midline {
                quadrants[2] += 1;
            }
        } 
        // right side
        else if robot.position.0 > y_midline {
            if robot.position.1 < x_midline {
                quadrants[1] += 1;
            }
            else if robot.position.1 > x_midline {
                quadrants[3] += 1;
            }
        }
    }
    return quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
}
