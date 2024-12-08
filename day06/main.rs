use ::utils;
use utils::{Direction, Point};

fn main() {
    let mut map = utils::read_input_lines("day06/input.txt")
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let start = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, c)| (i, j, c)))
        .find(|(_, _, c)| c == &&'^')
        .map(|(i, j, _)| (i, j))
        .unwrap();

    let start_pos = Point::new(start.0 as i32, start.1 as i32);

    let mut create_loop_points = vec![];
    walk(&mut map, start_pos, Direction::Up, &mut create_loop_points);

    // count 'X' in map
    let count = map
        .iter()
        .flat_map(|row| row.iter())
        .filter(|c| c == &&'X')
        .count();
    println!("The number of steps is: {}", count);

    let obstacle_at_starting_point = create_loop_points.contains(&start_pos) as usize;
    let create_loop_points_set = create_loop_points
        .iter()
        .collect::<std::collections::HashSet<_>>();
    println!(
        "Loop count: {}",
        create_loop_points_set.len() - obstacle_at_starting_point
    );
}

fn walk(map: &mut Vec<Vec<char>>, pos: Point, dir: Direction, create_loop_points: &mut Vec<Point>) {
    map[pos.i as usize][pos.j as usize] = 'X';
    let next_pos = pos.add(&dir.to_point());
    let next_pos_char = get_next_pos_char(pos, dir, map);
    if next_pos_char == 'O' {
        return;
    }

    if next_pos_char == '#' {
        let new_dir = dir.turn(Direction::Right);
        walk(map, pos, new_dir, create_loop_points);
    } else {
        if !create_loop_points.contains(&next_pos) && next_pos_char != 'X' {
            map[next_pos.i as usize][next_pos.j as usize] = '#';
            let visited = vec![];
            if check_loop(pos, dir, map, visited) {
                create_loop_points.push(next_pos);
            }
            map[next_pos.i as usize][next_pos.j as usize] = next_pos_char;
        }
        walk(map, next_pos, dir, create_loop_points);
    }
}

fn check_loop(
    pos: Point,
    dir: Direction,
    map: &Vec<Vec<char>>,
    mut visited: Vec<(Point, Direction)>,
) -> bool {
    let next_pos = pos.add(&dir.to_point());
    let next_pos_char = get_next_pos_char(pos, dir, map);

    if next_pos_char == 'O' {
        return false;
    }

    if visited.contains(&(pos, dir)) {
        return true;
    }
    visited.push((pos, dir));

    if next_pos_char == '#' {
        let new_dir = dir.turn(Direction::Right);
        return check_loop(pos, new_dir, map, visited);
    } else {
        return check_loop(next_pos, dir, map, visited);
    }
}

fn get_next_pos_char(pos: Point, dir: Direction, map: &Vec<Vec<char>>) -> char {
    let next_pos = pos.add(&dir.to_point());
    if next_pos.i < 0
        || next_pos.j < 0
        || next_pos.i >= map.len() as i32
        || next_pos.j >= map[0].len() as i32
    {
        return 'O';
    }
    map[next_pos.i as usize][next_pos.j as usize]
}
