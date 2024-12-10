use ::utils;
use strum::IntoEnumIterator;
use utils::{Direction, Point};

fn main() {
    let input = utils::read_input_lines("day10/sample.txt")
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let trailheads = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == 0)
                .map(move |(j, _)| (i, j))
        })
        .map(|(i, j)| Point::new(i as i32, j as i32))
        .collect::<Vec<Point>>();

    let mut sum = 0;
    let mut route_count = 0;
    for th in trailheads {
        let mut ending_points = vec![];
        let mut visited = vec![];
        calculate_score(&input, th, &mut visited, &mut ending_points, &mut route_count);
        sum += ending_points.len();
    }
    println!("Sum: {}", sum);
    println!("Route count: {}", route_count);
}

// bfs to find all 9's that can be reached from the trailhead
fn calculate_score(
    map: &Vec<Vec<i32>>,
    pos: Point,
    visited: &mut Vec<Point>,
    ending_points: &mut Vec<Point>,
    route_count: &mut i32
) {
    let pos_val = map[pos.i as usize][pos.j as usize];
    if pos_val == 9 {
        if !ending_points.contains(&pos) {
            ending_points.push(pos);
        }
        *route_count += 1;
        return;
    }
    // let mut visited = vec![];
    // visited.push(curr_pos);
    for dir in Direction::iter() {
        let next_pos = pos.add(&dir.to_point());
        if next_pos.in_bound(map) && map[next_pos.i as usize][next_pos.j as usize] == pos_val + 1 {
            visited.push(next_pos);
            calculate_score(map, next_pos, visited, ending_points, route_count);
            visited.pop();
        }
    }
}
