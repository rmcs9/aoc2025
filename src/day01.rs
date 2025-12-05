use util;

pub fn day01() {
    let data = util::get_data("data/day01.txt");

    println!("p1: {}", part1(&data));
    println!("p2: {}", part2(&data));
}


fn part1(data: &Vec<String>) -> u32 {
    let mut dial: i32 = 50;
    let mut result = 0;
    for line in data {
        let dir = line.chars().nth(0).unwrap();
        let delta: i32 = line[1..].parse().unwrap();
        if dir == 'R' {
            dial = (dial + delta) % 100;
        } else {
            dial = dial - delta;
            if dial < 0 {
                while dial < 0 {
                    dial = dial + 100;
                }
            }
        }
        if dial == 0 {
            result += 1;
        }
    }
    result
}

// listen i know this is dogwater/really lame... but ive had enough fights with this language for one day
fn part2(data: &Vec<String>) -> i32 {
    let mut dial = 50;
    let mut result = 0;

    for line in data {
        let dir = line.chars().nth(0).unwrap();
        let delta: i32 = line[1..].parse().unwrap();


        for _ in 0..delta {
            if dir == 'R' {
                dial = dial + 1;
                if dial == 100 {
                    dial = 0;
                    result += 1;
                }
            } else {
                dial = dial - 1;
                if dial == -1 {
                    dial = 99;
                }
                if dial == 0 {
                    result += 1;
                }
            }
        }
    }
    result
}



