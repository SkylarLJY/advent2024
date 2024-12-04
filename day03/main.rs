use ::utils;
use regex::Regex;

fn mul_sum(input: &str) -> i32 {
    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let re = Regex::new(pattern).unwrap();
    let mut sum = 0;
    for caps in re.captures_iter(&input) {
        let num1 = caps[1].parse::<i32>().unwrap();
        let num2 = caps[2].parse::<i32>().unwrap();
        sum += num1 * num2;
    }
    sum
}

fn main() {
    let input = utils::read_input("day03/input.txt");

    println!("Sum: {}", mul_sum(&input));

    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();
    let mut sum = 0;
    let mut do_idx_iter = do_pattern.find_iter(&input);
    let mut dont_idx_iter = dont_pattern.find_iter(&input);
    let mut do_idx = do_idx_iter.next();
    let mut dont_idx = dont_idx_iter.next();

    if dont_idx.is_none() {
        sum = mul_sum(&input);
        println!("Sum: {}", sum);
        return;
    }

    sum += mul_sum(&input[..dont_idx.unwrap().start()]);

    while true {
        match dont_idx {
            Some(dont_idx_match) => match do_idx {
                Some(mut do_idx_match) => {
                    if do_idx_match.start() < dont_idx_match.start() {
                        sum += mul_sum(&input[do_idx_match.end()..dont_idx_match.start()]);
                        while do_idx_match.start() < dont_idx_match.start() {
                            do_idx = do_idx_iter.next();
                            if do_idx.is_none() {
                                break;
                            }
                            do_idx_match = do_idx.unwrap();
                        }
                    } else {
                        dont_idx = dont_idx_iter.next();
                    }
                }
                None => {
                    break;
                }
            },
            None => {
                break;
            }
        }
    }

    if do_idx.is_some() && dont_idx.is_none() {
        sum += mul_sum(&input[do_idx.unwrap().end()..]);
    }

    println!("Sum: {}", sum);
}
