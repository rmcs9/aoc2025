use util;
use std::cmp::Ordering;
use std::cmp::max;

pub fn day05() {
    let data = util::get_data("data/day05.txt");
    let mut i = 0;
    // find where the input splits
    for line in &data {
        if line == "" {
            break;       
        }
        i = i + 1;
    }
    let ranges = sort_ranges(&data[..i]);
    
    println!("p1: {}", part1(&data[i + 1..], &ranges));
    println!("p2: {}", part2(&ranges));
}

fn part1(data: &[String], ranges: &Vec<(u64, u64)>) -> u64 {
    let mut result = 0;
    // parse the id numbers
    let nums_data: Vec<u64> = data.iter().map(|d| {
        return d.parse::<u64>().unwrap();
    }).collect();

    // figure out which range an ID number belongs in
    for num in nums_data {
        for range in ranges {
            if num < range.0 {
                break;
            }

            if num <= range.1 {
                result += 1;
                break;
            }
        }
    }
    return result;
}

fn part2(ranges: &Vec<(u64, u64)>) -> u64 {
    let mut result = 0;

    // just add up the total for each range
    for range in ranges {
        result += (range.1 - range.0) + 1;
    }
    return result;
}

fn sort_ranges(data: &[String]) -> Vec<(u64, u64)> {
    let ranges = &data;
    // map the ranges to u64 tuples
    let mut ranges: Vec<(u64, u64)> = ranges.iter().map(|range| {
        let range = String::from(range);
        let bounds: Vec<&str> = range.split('-').collect();
        let nums: (u64, u64) = (bounds[0].parse().unwrap(), bounds[1].parse().unwrap());
        return nums;
    }).collect();
    // sort them according to their lower bound
    ranges.sort_by(|a, b| {
        match a < b {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    });

    // condense the ranges together if they overlap
    let mut condensed_ranges: Vec<(u64, u64)> = Vec::new();
    for range in ranges {
        let pred = |old: &mut (u64, u64)| {
            range.0 <= old.1 
        };

        // pop the previous range if it overlaps with the new one
        let old_range = condensed_ranges.pop_if(pred);
        if old_range.is_some() {
            let old_range = old_range.unwrap();
            // enlarge the old range and push back onto the list
            condensed_ranges.push((old_range.0, max(old_range.1, range.1)));
        } else {
            condensed_ranges.push(range);
        }
    }
    
    return condensed_ranges;
}
