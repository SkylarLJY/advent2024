use std::vec;
use ::utils;

fn main() {
    let input = utils::read_input_lines("day07/input.txt");
    let mut sum = 0;

    for line in input {
        let (result, nums) = line.split_once(":").unwrap();
        let result = result.parse::<i64>().unwrap();
        let nums = nums
            .trim()
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if can_get_result(&nums, result) {
            sum += result;
        }
    }

    println!("The sum is: {}", sum);
}

fn can_get_result(nums: &Vec<i64>, result: i64) -> bool {
    if nums[0] as i64 > result {
        return false;
    }
    if nums.len() == 1 {
        return nums[0] as i64 == result;
    }

    let mut new_nums_plus = vec![nums[0] + nums[1]];
    new_nums_plus.extend_from_slice(&nums[2..]);
    let mut new_nums_mult = vec![nums[0] * nums[1]];
    new_nums_mult.extend_from_slice(&nums[2..]);
    let mut new_nums_concat = vec![(nums[0].to_string() + &nums[1].to_string())
        .parse::<i64>()
        .unwrap()];
    new_nums_concat.extend_from_slice(&nums[2..]);
    can_get_result(&new_nums_plus, result)
        || can_get_result(&new_nums_mult, result)
        || can_get_result(&new_nums_concat, result)
}
