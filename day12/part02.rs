use std::collections::{HashMap, HashSet};

pub fn part02(data2d: Vec<Vec<char>>) -> usize {
    let regions = get_regions(&data2d);

    let mut cost = 0;
    for region in regions.values() {
        cost += region.0 * region.2;
    }
    return cost;
}

fn get_regions(data2d: &Vec<Vec<char>>) -> HashMap<(usize, usize), (usize, usize, usize)> {
    let height = data2d.len();
    let width = data2d[0].len();

    let mut regions: HashMap<(usize, usize), (usize, usize, usize)> = HashMap::new(); 
    let mut visit_record: HashSet<(usize, usize)> = HashSet::new();

    for x in 0..width {
        for y in 0..height {
            if visit_record.contains(&(x, y)) { continue; } 

            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            
            flood_fill(data2d[y][x], (x, y), data2d, &mut visited, height, width);
            
            let area = visited.len();
            let perimeter = get_perimeter(&visited, width, height);
            let sides = get_sides(data2d[y][x], data2d, &visited, width, height);

            regions.insert((x, y), (area, perimeter, sides));

            visit_record.extend(visited);
        }
    }
    return regions;
}

fn flood_fill(character: char, start: (usize, usize), data2d: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>, height: usize, width: usize) {

    visited.insert(start);

    let new_points: Vec<(usize, usize)> = get_points(start, height, width).into_iter().filter(|coords| !visited.contains(coords) && data2d[coords.1][coords.0] == character).collect();

    if new_points.is_empty() { 
        return;
    }
    for point in new_points {
        flood_fill(character, point, data2d, visited, height, width);
    } 
}

fn get_perimeter(visited: &HashSet<(usize, usize)>, width: usize, height: usize) -> usize {
    let mut perimeter = 0;
    for point in visited {
        perimeter += 4 - get_points(*point, width, height).into_iter().filter(|point| visited.contains(point)).count();
    }
    return perimeter;
}

fn get_sides(character: char, grid: &Vec<Vec<char>>, visited: &HashSet<(usize, usize)>, width: usize, height: usize) -> usize {
    let mut sides = 0;
    
    for point in visited {
        let characters: Vec<char> = get_sides_points(*point).into_iter().map(|it| 
            if check_bounds(it, height, width) {
                grid[it.1 as usize][it.0 as usize]
            }
            // out of bounds
            else { '!' }
            ).collect();

        // no corners
        if characters[0] == character && characters[4] == character && characters[2] != character && characters[6] != character { continue; }
        if characters[2] == character && characters[6] == character && characters[0] != character && characters[4] != character { continue; }

        // check interior corners
        if characters[0] == character && characters[6] == character && characters[7] != character { sides += 1; }
        if characters[0] == character && characters[2] == character && characters[1] != character { sides += 1; }
        if characters[4] == character && characters[6] == character && characters[5] != character { sides += 1; }
        if characters[4] == character && characters[2] == character && characters[3] != character { sides += 1; }

        // check exterior corners
        if characters[0] != character && characters[6] != character { sides += 1; }
        if characters[0] != character && characters[2] != character { sides += 1; }
        if characters[4] != character && characters[6] != character { sides += 1; }
        if characters[4] != character && characters[2] != character { sides += 1; }
    }
    return sides;
}

fn get_points(point: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    // get points above, below, left and right
    let list = vec![
        (point.0 as i32, point.1 as i32 - 1), (point.0 as i32, point.1 as i32 + 1), (point.0 as i32 - 1, point.1 as i32), (point.0 as i32 + 1, point.1 as i32)
    ];
    list.into_iter().filter(|it| check_bounds(*it, height, width)).map(|it| (it.0 as usize, it.1 as usize)).collect()
}

fn get_sides_points(point: (usize, usize)) -> [(i32, i32); 8] {
    // get points north, northeast, east, southeast, south, southwest, west and northwest
    [
        (point.0 as i32, point.1 as i32 - 1), (point.0 as i32 + 1, point.1 as i32 - 1),
        (point.0 as i32 + 1, point.1 as i32), (point.0 as i32 + 1, point.1 as i32 + 1),
        (point.0 as i32, point.1 as i32 + 1), (point.0 as i32 - 1, point.1 as i32 + 1),
        (point.0 as i32 - 1, point.1 as i32), (point.0 as i32 - 1, point.1 as i32 - 1)
    ]
}

fn check_bounds(point: (i32, i32), height: usize, width: usize) -> bool {
    point.0 >= 0 && point.0 < width as i32 && point.1 >= 0 && point.1 < height as i32
}
