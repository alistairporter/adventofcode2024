use std::collections::HashSet;

pub fn part02(grid: &Vec<Vec<i32>>, trailheads: &Vec<(usize, usize)>, width: usize, height: usize) -> usize {
    let mut result = 0;

    for trail in trailheads {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        result += find_hikes(grid, &mut set, *trail, 0, width, height);
    }
    return result;
}

fn find_hikes(grid: &Vec<Vec<i32>>, set: &mut HashSet<(usize, usize)>, location: (usize, usize), elevation: i32, width: usize, height: usize) -> usize {

    if elevation == 9 {
        set.insert(location);
        return 1;
    }

    else {
        let points = get_points(location, height, width);
        let mut hikes = 0;
        for point in points {
            if grid[point.1][point.0] == elevation + 1 {
                hikes += find_hikes(grid, set, point, elevation + 1, width, height);
            }
        }
        return hikes;
    }
}

fn get_points(point: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {

    let list = vec![
        (point.0 as i32, point.1 as i32 - 1), (point.0 as i32, point.1 as i32 + 1), (point.0 as i32 - 1, point.1 as i32), (point.0 as i32 + 1, point.1 as i32)
    ];

    list.into_iter().filter(|it| check_bounds(*it, height, width)).map(|it| (it.0 as usize, it.1 as usize)).collect()
}

fn check_bounds(point: (i32, i32), height: usize, width: usize) -> bool {
    point.0 >= 0 && point.0 < width as i32 && point.1 >= 0 && point.1 < height as i32
}
