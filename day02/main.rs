use ::utils;

fn is_safe(nums: &Vec<i32>) -> bool {
    nums.windows(2)
        .all(|w| w[0] - w[1] >= 1 && w[0] - w[1] <= 3)
        || nums
            .windows(2)
            .all(|w| w[1] - w[0] >= 1 && w[1] - w[0] <= 3)
}

fn check_unsafe_line(nums: &Vec<i32>) -> bool {
    for i in 0..nums.len() {
        let mut new_line = nums.clone();
        new_line.remove(i);
        if is_safe(&new_line) {
            return true;
        }
    }
    false
}

pub fn main() {
    let input = utils::read_input_lines("day02/input.txt");
    let res = input.iter().fold(0, |mut acc, line| {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let safe = is_safe(&nums);
        acc = if safe { acc + 1 } else { acc };
        if !safe {
            acc = if check_unsafe_line(&nums) {
                acc + 1
            } else {
                acc
            };
        }
        acc
    });
    println!("Result: {}", res);
}
