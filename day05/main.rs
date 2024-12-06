use std::collections::HashMap;

use ::utils;

fn main() {
    let input = utils::read_input("day05/input.txt");
    // split at enpty line
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let rules = input[0].split("\n").collect::<Vec<&str>>();
    let orders = input[1].split("\n").collect::<Vec<&str>>();
    let mut value_order_map: HashMap<i32, Vec<i32>> = std::collections::HashMap::new();

    for r in rules {
        let (x, y) = r.split_at(r.find("|").unwrap());
        let x = x.trim().parse::<i32>().unwrap();
        let y = y[1..].trim().parse::<i32>().unwrap();
        value_order_map.entry(x).or_insert(vec![]).push(y);
    }

    let mut mid_sum = 0;
    let mut incorrect_list = vec![];
    for o in orders {
        let nums = o
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let correct = nums.iter().enumerate().fold(true, |mut acc, (i, n)| {
            if !value_order_map.contains_key(n) {
                value_order_map.insert(*n, vec![]);
            }
            let arr = value_order_map.get(n).unwrap();
            acc = acc && nums[i + 1..].iter().all(|x| arr.contains(x));
            acc
        });
        if correct {
            mid_sum += nums[nums.len() / 2];
        } else {
            incorrect_list.push(nums);
        }
    }
    println!("Part 1: {}", mid_sum);

    mid_sum = 0;
    for nums in incorrect_list {
        for n in &nums {
            // if half of the rest of the numebrs are before n and half after n then it's the middle
            let (before, after) = nums.iter().fold((0, 0), |(b, a), cur| {
                if cur == n {
                    return (b, a);
                }
                if value_order_map.get(&n).unwrap().contains(&cur) {
                    (b, a + 1)
                } else {
                    (b + 1, a)
                }
            });
            if before == after {
                mid_sum += n;
                break;
            }
        }
    }
    println!("Part 2: {}", mid_sum);
}
