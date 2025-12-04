use util;

fn main() {
    let data = util::get_data("data/day04.txt");

    // turn each string into a char vector cuz string suck
    let mut data_chars: Vec<Vec<char>> = Vec::new();
    for line in &data {
        let line_chars: Vec<char> = line.chars().collect();
        data_chars.push(line_chars);
    }

    let mut q: Vec<(usize, usize)> = Vec::new();
    println!("p1: {}", part1(&mut data_chars, &mut q));
    println!("p2: {}", part2(&mut data_chars, &mut q));
}

fn part1(data: &mut Vec<Vec<char>>, q: &mut Vec<(usize, usize)>) -> u32 {
    let mut result = 0;
    // for every coordinate
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            let ch = data[y][x];
            if ch != '@' { continue }
            // find all the surrounding rolls
            let mut surrounds = 0;
            for dir in DIRS {
                let new_x = (x as i32 + dir.0) as usize;
                let new_y = (y as i32 + dir.1) as usize;

                if !in_bounds(new_x, new_y, &data) { continue }
                let peek = data[new_y][new_x];
                if peek != '@' { continue }
                surrounds = surrounds + 1;
                if surrounds > 3 { break }
            }
            if surrounds < 4 {
                result = result + 1;
                // push each removed roll to a queue for part 2
                q.push((x, y));
            }
        }
    }
    return result;
}

fn part2(data: &mut Vec<Vec<char>>, q: &mut Vec<(usize, usize)>) -> u32 {
    let mut result = 0;
    // BFS
    while !q.is_empty() {
        let (x, y) = q.pop().unwrap();
        if data[y][x] != '@' { continue }
        let mut adjacent: Vec<(usize, usize)> = Vec::new();
        for dir in DIRS {
            let new_x = (x as i32 + dir.0) as usize;
            let new_y = (y as i32 + dir.1) as usize;

            if !in_bounds(new_x, new_y, &data) { continue }
            let peek = data[new_y][new_x];
            if peek != '@' { continue }
            adjacent.push((new_x, new_y));
        }

        if adjacent.len() < 4 {
            data[y][x] = 'x';
            result += 1;
            for ats in adjacent {
                q.push(ats);
            }
        }
    }
    return result;
}

fn in_bounds(x: usize, y: usize, data: &Vec<Vec<char>>) -> bool {
    // x and y have a probability of being wrapped around (unsigned 0 - 1) but it shouldnt really matter
    return y < data.len() && x < data[y].len();
}


const DIRS: [(i32, i32); 8] = [
    (-1, -1), 
    (-1, 0), 
    (-1, 1), 
    (1, -1), 
    (1, 0), 
    (1, 1), 
    (0, 1), 
    (0, -1),
];
