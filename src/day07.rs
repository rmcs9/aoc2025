use std::collections::HashSet;
use std::cmp::max;

pub fn day07() {
    let data = util::get_data("data/day07.txt");

    let start = find_start(&data);

    println!("p1: {}", part1(&data, &start));
    println!("p2: {}", part2(&data, &start));
}

fn part1(data: &Vec<String>, start: &(usize, usize)) -> u32 {
    let mut mem: HashSet<(usize, usize)> = HashSet::new();
    let mut beams: Vec<usize> = Vec::new();
    let mut data: Vec<Vec<char>> = data.iter().map(|l| l.chars().collect()).collect();
    beams.push(start.0);
    
    let mut y = 0;
    for line in &mut data {
        let mut enq: Vec<usize> = Vec::new();
        for beam in beams.drain(..) {
            if line[beam] == '^' {

                if !mem.contains(&(beam, y)) {
                    mem.insert((beam, y));
                    enq.push(beam + 1);
                    enq.push(beam - 1);
                }
            } else {
                line[beam] = '|';
                enq.push(beam);
            }
        }

        beams.append(&mut enq);
        y += 1;
    }

    return mem.len() as u32;
}

fn part2(data: &Vec<String>, start: &(usize, usize)) -> usize {
    let data: Vec<Vec<char>> = data.iter().map(|l| l.chars().collect()).collect();
    let mut mem: Vec<Vec<usize>> = data.iter().map(|l| l.iter().map(|_| 0).collect()).collect();

    mem[start.1][start.0] = 1;

    for y in 1..data.len() {
        for x in 0..data[0].len() {
            let c = data[y][x];

            if c == '.' {
                let old = mem[y][x]; 
                let prev = mem[y - 1][x];
                mem[y][x] = old + prev;
                continue;
            }

            if c == '^' {
                let old_m1 = mem[y][x - 1];
                let old_p1 = mem[y][x + 1];
                let prev = mem[y - 1][x];

                mem[y][x + 1] = max(old_p1 + prev, old_p1);
                mem[y][x - 1] = max(old_m1 + prev, old_m1);
            }
        }
    }

    mem[data.len() - 1].iter().sum()
}

fn find_start(data: &Vec<String>) -> (usize, usize) {
    return (data[0].find('S').unwrap(), 0);
}
