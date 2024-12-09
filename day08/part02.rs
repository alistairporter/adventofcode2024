use std::collections::{HashMap, HashSet};
use core::f32;

fn check_bounds(point: (i32, i32), height: usize, width: usize) -> bool {
    point.0 >= 0 && point.0 < width as i32 && point.1 >= 0 && point.1 < height as i32
}

pub fn part02(antennas: &HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    // use a HashSet for antinodes because we want unique number
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    // for every antenna type
    for locations in antennas.values() {
        for antenna_1 in 0..locations.len() - 1 {
            for antenna_2 in antenna_1 + 1..locations.len() {
                // for every unique combination of two antennas
                let (x_1, y_1) = locations[antenna_1];
                let (x_2, y_2) = locations[antenna_2];
                let slope: f32 = (y_2 - y_1) as f32 / (x_2 - x_1) as f32;
                let x_distance = x_2 - x_1;

                // insert antinodes going left until we run off the map
                let mut multiplier: i32 = 0;
                loop {
                    let antinode = (x_1 -  x_distance * multiplier, y_1 - (slope * (x_distance * multiplier) as f32) as i32); 
                    if check_bounds(antinode, height, width) { 
                        antinodes.insert(antinode); 
                        multiplier += 1;
                    } 
                    else { 
                        break; 
                    }
                } 
                // insert antinodes going right until we run off the map
                multiplier = 1;
                loop {
                    let antinode = (x_1 + x_distance * multiplier, y_1 + (slope * (x_distance * multiplier) as f32) as i32); 
                    if check_bounds(antinode, height, width) { 
                        antinodes.insert(antinode); 
                        multiplier += 1;
                    } 
                    else { 
                        break; 
                    }
               }
            }
        }
    }
    return antinodes.len();
}
