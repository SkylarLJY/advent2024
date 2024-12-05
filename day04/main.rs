use ::utils;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }

    fn get_on_grid(&self, grid: &Vec<Vec<char>>) -> char {
        grid[self.x as usize][self.y as usize]
    }
    fn is_in_range(&self, row_cnt: usize, col_cnt: usize) -> bool {
        self.x >= 0 && self.x < row_cnt as i32 && self.y >= 0 && self.y < col_cnt as i32
    }
}
fn main() {
    let input = utils::read_input_lines("day04/input.txt");
    let mut grid = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let dir = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 'X' {
                continue;
            }
            for d in dir.iter() {
                res += search(i, j, 'X', &mut grid, d);
            }
        }
    }
    println!("P1 Result: {}", res);

    res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 'A' {
                continue;
            }
            let p = Point::new(i as i32, j as i32);
            let upper_left = p.add(&Point::new(-1, 1));
            let upper_right = p.add(&Point::new(-1, -1));
            let lower_left = p.add(&Point::new(1, 1));
            let lower_right = p.add(&Point::new(1, -1));
            if [upper_left, upper_right, lower_left, lower_right]
                .iter()
                .any(|point| !point.is_in_range(grid.len(), grid[0].len()))
            {
                continue;
            }
            // if upper left and lower right one is M and one is S
            let cross1 = [
                upper_left.get_on_grid(&grid),
                lower_right.get_on_grid(&grid),
            ];
            let cross2 = [
                upper_right.get_on_grid(&grid),
                lower_left.get_on_grid(&grid),
            ];
            if cross1.contains(&'M')
                && cross1.contains(&'S')
                && cross2.contains(&'M')
                && cross2.contains(&'S')
            {
                res += 1;
            }
        }
    }
    println!("P2 Result: {}", res);
}

fn search(i: usize, j: usize, target: char, grid: &mut Vec<Vec<char>>, dir: &(i32, i32)) -> i32 {
    if grid[i][j] != target {
        return 0;
    }
    if target == 'S' {
        return 1;
    }

    let mut res = 0;
    let (dx, dy) = dir;
    let ni = i as i32 + dx;
    let nj = j as i32 + dy;
    if ni < 0 || nj < 0 || ni >= grid.len() as i32 || nj >= grid[0].len() as i32 {
        return 0;
    }
    let next_target = match target {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => panic!("Invalid target"),
    };
    res += search(ni as usize, nj as usize, next_target, grid, dir);

    res
}
