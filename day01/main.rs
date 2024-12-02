use std::collections::{HashMap, HashSet};

use utils;

fn main() {
    let lines = utils::read_input_lines("day01/input.txt");
    // new vec of numbers
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        assert!(nums.len() == 2);
        list1.push(nums[0]);
        list2.push(nums[1]);
    }

    list1.sort();
    list2.sort();

    let mut dist_sum = 0;

    for (num1, num2) in list1.iter().zip(list2.iter()) {
        dist_sum += (num1 - num2).abs();
    }

    println!("Sum of distances: {}", dist_sum);

    let list2_freq_map = list2.iter().fold(HashMap::new(), |mut acc, num| {
        let occurance = acc.entry(num).or_insert(0);
        *occurance += 1;
        acc
    });

    let list1_num_set: HashSet<_> = HashSet::from_iter(list1.iter());

    let mut score = 0;
    for num in list1_num_set.iter() {
        let freq = list2_freq_map.get(num).unwrap_or(&0);
        score += freq * *num;
    }
    println!("Score: {}", score); 
}
