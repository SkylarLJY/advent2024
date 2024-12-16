use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::vec;

use ::utils;
use regex::Regex;
use utils::Point;

struct MachineConfig {
    button_a: Point,
    button_b: Point,
    target: Point,
}

#[derive(Eq, PartialEq, Debug)]
struct Node {
    pos: Point,
    a_press_count: i32,
    b_press_count: i32,
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_cost = self.a_press_count * BUTTON_A_COST + self.b_press_count * BUTTON_B_COST;
        let other_cost = other.a_press_count * BUTTON_A_COST + other.b_press_count * BUTTON_B_COST;
        self_cost.cmp(&other_cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const BUTTON_A_COST: i32 = 3;
const BUTTON_B_COST: i32 = 1;

fn main() {
    let button_a_pattern = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let button_b_pattern = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_pattern = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let input = utils::read_input("day13/sample.txt")
        .split("\n\n")
        .map(|str| {
            let config = str.split("\n").collect::<Vec<&str>>();
            let btn_a = parse_config(&button_a_pattern, config[0]);
            let btn_b = parse_config(&button_b_pattern, config[1]);
            let prize = parse_config(&prize_pattern, config[2]);
            MachineConfig {
                button_a: btn_a,
                button_b: btn_b,
                target: prize,
            }
        })
        .collect::<Vec<MachineConfig>>();
    let res = input
        .iter()
        .map(|m| count_tokens(m))
        .filter(|x| *x != -1)
        .sum::<i64>();
    println!("The total number of tokens is: {}", res);
}

fn parse_config(pattern: &Regex, haystack: &str) -> Point {
    let cap = pattern.captures(haystack).unwrap();
    Point::new(
        cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
    )
}

// count the shortest path in a weighted graph
fn count_tokens(config: &MachineConfig) -> i64 {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(Node {
        pos: Point::new(0, 0),
        a_press_count: 0,
        b_press_count: 0,
    }));
    let mut visited = vec![vec![false; 105]; 105];
    while !heap.is_empty() {
        let node = heap.pop().unwrap().0;
        if visited[node.a_press_count as usize][node.b_press_count as usize] {
            continue;
        }
        visited[node.a_press_count as usize][node.b_press_count as usize] = true;
        if node.a_press_count >= 100 || node.b_press_count >= 100 {
            continue;
        }
        if node.pos.i == node.pos.j && node.pos.i % 10 == 0 {}
        if node.pos == config.target {
            return (node.a_press_count * BUTTON_A_COST + node.b_press_count * BUTTON_B_COST)
                as i64;
        }
        let new_point_a = node.pos.add(&config.button_a);
        if new_point_a.i <= config.target.i && new_point_a.j <= config.target.j {
            heap.push(Reverse(Node {
                pos: new_point_a,
                a_press_count: node.a_press_count + 1,
                b_press_count: node.b_press_count,
            }));
        }
        let new_point_b = node.pos.add(&config.button_b);
        if new_point_b.i <= config.target.i && new_point_b.j <= config.target.j {
            heap.push(Reverse(Node {
                pos: new_point_b,
                a_press_count: node.a_press_count,
                b_press_count: node.b_press_count + 1,
            }));
        }
    }
    -1
}
