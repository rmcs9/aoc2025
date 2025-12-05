use util;
use std::collections::HashSet;

pub fn day02() {
    let data = util::get_data("data/day02.txt");
    let line = &data[0];
    let mut ranges: Vec<(String, String, u64, u64)> = Vec::new();
    let raw_ranges: Vec<&str> = line.split(',').collect();
    for range in raw_ranges {
        let bounds: Vec<&str> = range.split('-').collect();
        let strs = (String::from(bounds[0]), String::from(bounds[1]));
        let nums: (u64, u64) = (bounds[0].parse().unwrap(), bounds[1].parse().unwrap());
        let range: (String, String, u64, u64) = (strs.0, strs.1, nums.0, nums.1);
        ranges.push(range);
    }
    //make ranges immutable
    let ranges = ranges;

    println!("p1: {}", part1(&ranges));
    println!("p2: {}", part2(&ranges));
}

fn part1(data: &Vec<(String, String, u64, u64)>) -> u64 {
    let mut result = 0;
    for range in data {
        let (min_str, max_str, min_num, max_num) = range; 
        // place difference refers to the difference between the number of digits in the min - max
        let place_difference = max_str.len() - min_str.len();
        
        // if the entire range falls on a numerical length that is odd,
        // there are no possible invalid ids
        if place_difference == 0 && max_str.len() % 2 == 1 {
            //skip this range
            continue
        }

        // if place difference is 0  
        if place_difference == 0 {
            result += find_invalid(&range);
        } else if place_difference == 1 { // if the place difference is 1, divide the range into
                                          // pieces at the digit boundry. for example on the range:
                                          // 991 - 1234, the ranges get divided into 991 - 999 and
                                          //     1000 - 1234
            let base: u64 = 10;
            // since numbers with an odd number of digits cant be considered, only do the range
            // with the even number of digits
            if max_str.len() % 2 == 1 {
                let adjusted_max_bound = max_num - ((max_num - base.pow((max_str.len() - 1) as u32)) + 1);
                let range = (min_str.clone(), adjusted_max_bound.to_string(), min_num.clone(), adjusted_max_bound);
                result += find_invalid(&range);
            } 

            if max_str.len() % 2 != 1 {
                let adjusted_min_bound = base.pow(min_str.len() as u32);    
                let range = (adjusted_min_bound.to_string(), max_str.clone(), adjusted_min_bound, max_num.clone());
                result += find_invalid(&range);
            }
        }
    }
    result
}

fn part2(data: &Vec<(String, String, u64, u64)>) -> u64 {
    let mut result = 0;
    for range in data {
        let (min_str, max_str, min_num, max_num) = range; 
        let place_difference = max_str.len() - min_str.len();

        // if place difference is 0
        if place_difference == 0 {
            result += find_invalid_2(&range);
        } else if place_difference == 1 { 
            // odd ranges can be considered now so split at the digit boundry and try everything
            let base: u64 = 10;
            let adjusted_max_bound = max_num - ((max_num - base.pow((max_str.len() - 1) as u32)) + 1);
            //ummmmmmmm
            let range = (min_str.clone(), adjusted_max_bound.to_string(), min_num.clone(), adjusted_max_bound);
            result += find_invalid_2(&range);
            let adjusted_min_bound = base.pow(min_str.len() as u32);    
            //ummmmmmmmmmmmmmmmmmmmmmm
            let range = (adjusted_min_bound.to_string(), max_str.clone(), adjusted_min_bound, max_num.clone());
            result += find_invalid_2(&range);
        }
    }

    return result
}

fn find_invalid(range: &(String, String, u64, u64)) -> u64 {
    let (min_str, max_str, min_num, max_num) = range; 
    let mut result = 0;
    // determine the length of the repeated pattern
    let section_length = max_str.len() / 2;

    let min_sig_str: &str = &min_str[0..section_length];
    let max_sig_str: &str = &max_str[0..section_length];

    let min_sig_num: u64 = min_sig_str.parse().unwrap(); 
    let max_sig_num: u64 = max_sig_str.parse().unwrap(); 

    for i in min_sig_num..max_sig_num + 1 {
        // repeat the target number twice and check if it fits within the required range
        let invalid_str = format!("{}{}", i.to_string(), i.to_string()); 
        let invalid_num: u64 = invalid_str.parse::<u64>().unwrap();

        if invalid_num <= *max_num && invalid_num >= *min_num {
            result += invalid_num;
        }
    }
    return result
}

// function assumes the place difference between min and max is the same 
fn find_invalid_2(range: &(String, String, u64, u64)) -> u64 {
    let mut mem: HashSet<u64> = HashSet::new();
    let mut result = 0;
    let (min_str, max_str, min_num, max_num) = range; 
    // determine if the digit length is even or odd
    let parity = min_str.len() % 2 == 0; 

    // determine the max possible length of a repeated pattern
    let max_pattern_length = if parity { min_str.len() / 2 } else { min_str.len() / 3 };    
    // from the size of the max pattern, work down attempting to make smaller and smaller patterns
    // each time
    for i in (1..max_pattern_length + 1).rev() {
        if min_str.len() % i != 0 {
            continue;
        }
        let min_sig_str = &min_str[0..i];
        let max_sig_str = &max_str[0..i];

        let min_sig_num: u64 = min_sig_str.parse().unwrap();
        let max_sig_num: u64 = max_sig_str.parse().unwrap();

        for j in min_sig_num..max_sig_num + 1 {

            let mut invalid_str = String::new(); 
            // create a string with the repeating pattern enough times to fill the target amount of
            // digits. 
            for _ in 0..min_str.len() / i {
                invalid_str.push_str(&j.to_string());
            }
            let invalid_num: u64 = invalid_str.parse::<u64>().unwrap();

            if invalid_num <= *max_num && invalid_num >= *min_num && !mem.contains(&invalid_num) {
                result += invalid_num;
                mem.insert(invalid_num);
            }
        }
    }
    return result;
}
