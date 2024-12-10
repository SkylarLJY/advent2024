use std::fs;
use strum_macros::EnumIter; 
pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Unable to read input file")
}

pub fn read_input_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Unable to read input file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub i: i32, // row index
    pub j: i32, // column index
}

impl Point {
    pub fn new(i: i32, j: i32) -> Point {
        Point { i, j }
    }

    pub fn add(&self, other: &Point) -> Point {
        Point::new(self.i + other.i, self.j + other.j)
    }

    pub fn minus(&self, other: &Point) -> Point {
        Point::new(self.i - other.i, self.j - other.j)
    }

    pub fn in_bound<T>(&self, input: &Vec<Vec<T>>) -> bool {
        self.i >= 0 && self.i < input.len() as i32 && self.j >= 0 && self.j < input[0].len() as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    pub fn to_point(&self) -> Point {
        match self {
            Direction::Up => Point::new(-1, 0),
            Direction::Down => Point::new(1, 0),
            Direction::Left => Point::new(0, -1),
            Direction::Right => Point::new(0, 1),
        }
    }

    pub fn turn(&self, dir: Direction) -> Direction {
        match self {
            Direction::Up => match dir {
                Direction::Left => Direction::Left,
                Direction::Right => Direction::Right,
                _ => panic!("Invalid turn"),
            },
            Direction::Down => match dir {
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
                _ => panic!("Invalid turn"),
            },
            Direction::Left => match dir {
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
                _ => panic!("Invalid turn"),
            },
            Direction::Right => match dir {
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
                _ => panic!("Invalid turn"),
            },
        }
    }
}
