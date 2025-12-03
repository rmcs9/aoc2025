use util;


fn main() {
    let data = util::get_data("data/day03.txt");

    println!("p1: {}", part1(&data));
    println!("p2: {}", part2(&data));
}

fn part1(data: &Vec<String>) -> i32 {
    let mut result = 0;

    for arr in data {
        // parse into integers
        let nums: Vec<i32> = arr.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        // stack for keeping track of previous best pairs
        let mut possible_results: Vec<(i32, i32)> = Vec::new();
        let mut first = -1;
        let mut second = -1;
        for num in nums {
            if num > first {
                // if uve already found a good number, its more than likely that uve found 2 good
                // numbers. save them for later just in case u are too far in the array to find 2
                // good numbers again
                if first != -1 {
                    possible_results.push((first, num));
                }
                first = num;
                second = -1;
            } else if num > second {
                second = num;
            }
        }

        // if u werent able to find a second good number, take the previous pair
        if second == -1 {
            (first, second) = possible_results.pop().unwrap();
        }

        let arr_result = (first * 10) + second;
        result += arr_result;
    }
    return result;
}


fn part2(data: &Vec<String>) -> u64 {
    let mut result = 0;

    for arr in data {
        let nums: Vec<u64> = arr.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
        result += find_next_num(&nums, 12, 0);
    }

    return result;
}

fn find_next_num(nums: &Vec<u64>, spots_remaining: usize, starting_index: usize) -> u64 {
    // determine the amount of numbers that you are able to choose from based on where u are in the
    // array and the amount of spots remaining
    let working_size = nums.len() - spots_remaining - starting_index + 1;
    // slice the portion of the array that you are able to pick from
    let range = &nums.as_slice()[starting_index..(starting_index + working_size)];
    // find the max number and index in the sliced portion
    let mut max_num: u64 = 0;
    let mut index = 0;
    for i in 0..range.len() {
        if range[i] > max_num {
            max_num = range[i];
            index = i;
        }
    }

    // decrement the amount of spots remaining 
    let new_spots_remaining = spots_remaining - 1;
    // update the starting index based on where u took the max number from
    let new_starting_index = starting_index + index + 1; 
    let base: u64 = 10;
    // base case: no spots left to fill 
    if new_spots_remaining == 0 {
        return max_num * base.pow(new_spots_remaining as u32); 
    }
    return (max_num * base.pow(new_spots_remaining as u32)) + find_next_num(&nums, new_spots_remaining, new_starting_index);
}
