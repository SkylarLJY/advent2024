use std::vec;

use ::utils;
use strum::IntoEnumIterator;
use utils::{Direction, Point};

fn main() {
    let input = utils::read_input_lines("day12/input.txt")
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if visited[i][j] {
                continue;
            }
            let mut cluster = vec![];
            find_cluster(
                &input,
                &mut visited,
                Point::new(i as i32, j as i32),
                input[i][j],
                &mut cluster,
            );
            // println!("area: {:?}", cluster.len());
            let peri = calculate_perimeter(&input, &cluster);
            // println!("peri: {:?}", peri);
            sum += peri * cluster.len() as i64;
            // println!("{} - {}", input[i][j],peri * cluster.len() as i64);
        }
    }
    println!("area * peri sum: {:?}", sum);
}

fn find_cluster(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    pos: Point,
    cur_char: char,
    cluster: &mut Vec<Point>,
) {
    if *pos.get_in_map(visited) || *pos.get_in_map(map) != cur_char {
        return;
    }
    cluster.push(pos);
    visited[pos.i as usize][pos.j as usize] = true;
    for dir in Direction::iter() {
        let next = pos.add(&dir.to_point());
        if next.in_bound(map) && !*next.get_in_map(visited) {
            find_cluster(map, visited, next, cur_char, cluster);
        }
    }
}

fn calculate_perimeter(map: &Vec<Vec<char>>, cluster: &Vec<Point>) -> i64 {
    let mut sum = 0;
    for o in cluster {
        for dir in Direction::iter() {
            let next = o.add(&dir.to_point());
            if next.in_bound(map) && cluster.contains(&next) {
                continue;
            }
            sum += 1;
        }
    }
    sum
}


