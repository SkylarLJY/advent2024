use std::collections::HashMap;

use ::utils;
use utils::Point;

fn main() {
    let input = utils::read_input_lines("day08/input.txt")
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut char_loc_map: HashMap<char, Vec<Point>> = HashMap::new();
    input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, c)| (i, j, c)))
        .filter(|(_, _, c)| c.is_alphabetic() || c.is_numeric())
        .for_each(|(i, j, c)| {
            let point = Point::new(i as i32, j as i32);
            char_loc_map.entry(*c).or_insert(vec![]).push(point);
        });

    let mut antinodes = vec![vec![false; input[0].len()]; input.len()];
    for (_, locs) in char_loc_map.iter() {
        // get_antinodes(locs, &mut antinodes, &input);
        if locs.len() == 1 {
            continue;
        }
        get_antinodes_with_resonance(locs, &mut antinodes, &input);
    }

    let antinode_count = antinodes
        .iter()
        .flat_map(|row| row.iter())
        .filter(|x| **x)
        .count();

    println!("The number of antinodes is: {}", antinode_count);
}

fn get_antinodes(locs: &Vec<Point>, antinodes: &mut Vec<Vec<bool>>, map: &Vec<Vec<char>>) {
    for i in 0..locs.len() {
        for j in i + 1..locs.len() {
            let diff = locs[i].minus(&locs[j]);
            let node1 = locs[j].minus(&diff);
            let node2 = locs[i].add(&diff);
            if node1.in_bound(&map){
                antinodes[node1.i as usize][node1.j as usize] = true;
            }
            if node2.in_bound(&map) {
                antinodes[node2.i as usize][node2.j as usize] = true;
            }
        }
    }
}

fn get_antinodes_with_resonance(locs: &Vec<Point>, antinodes: &mut Vec<Vec<bool>>, map: &Vec<Vec<char>>) {
    for i in 0..locs.len() {
        for j in i + 1..locs.len() {
            antinodes[locs[i].i as usize][locs[i].j as usize] = true;
            antinodes[locs[j].i as usize][locs[j].j as usize] = true;
            let diff = locs[i].minus(&locs[j]);
            let mut node1 = locs[j].minus(&diff);
            while node1.in_bound(&map) {
                antinodes[node1.i as usize][node1.j as usize] = true;
                node1 = node1.minus(&diff);
            }

            let mut node2 = locs[i].add(&diff);
            while node2.in_bound(&map) {
                antinodes[node2.i as usize][node2.j as usize] = true;
                node2 = node2.add(&diff);
            }
        }
    }
}

