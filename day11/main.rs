use std::{collections::HashMap, vec};

use ::utils;

fn main() {
    let input = utils::read_input("day11/input.txt")
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut val_count_map = HashMap::new();
    input.iter().for_each(|x| {
        let count = val_count_map.entry(*x).or_insert(0);
        *count += 1;
    });

    const NUM_REP: i32 = 75;
    let mut nums = val_count_map.keys().map(|x| *x).collect::<Vec<i64>>();
    for _ in 0..NUM_REP {
        let mut temp = HashMap::new();
        nums.iter().for_each(|x| {
            let parent_count = val_count_map.get(x).unwrap();

            blink(x).iter().for_each(|y| {
                let count = temp.entry(*y).or_insert(0);
                *count += *parent_count;
            });
        });
        val_count_map = temp.clone();
        nums = val_count_map.keys().map(|k| *k).collect::<Vec<i64>>();

        // println!("{:?}", nums);
        // println!("{:?}", val_count_map);
    }

    let sum: i64 = val_count_map.values().sum();
    println!("{:?}", sum);

    // let mut res = input.clone();
    // for _ in 0..NUM_REP {
    //     res = res.iter().flat_map(|x| blink(x)).collect::<Vec<i64>>();
    // }
    // println!("{:?}", res.len());
}

fn blink(x: &i64) -> Vec<i64> {
    if *x == 0 {
        return vec![1];
    }

    let x_len = ((*x as f64).log10() as usize) + 1;
    if x_len % 2 == 0 {
        let n1 = x / (10i64.pow(x_len as u32 / 2)) as i64;
        let n2 = x % (10i64.pow(x_len as u32 / 2)) as i64;
        return vec![n1, n2];
    }
    vec![x * 2024]
}
