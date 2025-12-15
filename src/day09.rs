use std::sync::Arc;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::cmp::max;
use std::thread;
use std::ops::Range;


pub fn day09() {
    let data = util::get_data("data/day09.txt");
    let data: Vec<(i64, i64)> = data.iter().map(|e| {
        let mut splt = e.split(',');
        let coord = (splt.next().unwrap().parse().unwrap(), splt.next().unwrap().parse().unwrap());
        return coord;
    }).collect();

    let mut min: (i64, i64) = (2_000_000_000, 2_000_000_000);
    let mut max: (i64, i64) = (-1, -1);
    for c in &data {
        if c.0 < min.0 {
            min.0 = c.0;
        } else if c.0 > max.0 {
            max.0 = c.0;
        }

        if c.1 < min.1 {
            min.1 = c.1;   
        } else if c.1 > max.1 {
            max.1 = c.1;
        }
    }

    println!("p1: {}", part1(&data));
    println!("p2: {}", part2(&data, min, max));
}

fn part1(data: &Vec<(i64, i64)>) -> i64 {
    let mut max = 0;
    for i in 0..data.len() {
        let c1 = data[i];
        for j in 0..data.len() {
            if i == j {
                continue;
            }

            let c2 = data[j];

            let area = ((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs() + 1);
            if area > max {
                max = area;
            }
        }
    }
    return max;
}

fn collect_ranges(size: usize, split: usize) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut current = 0;

    while current < size {
        let end = (current  + split).min(size);
        ranges.push(current..end);
        current = end;
    }

    return ranges;
}

fn part2(data: &Vec<(i64, i64)>, min_el: (i64, i64), max_el: (i64, i64)) -> i64 {
    let actual_y = (max_el.1 - min_el.1 + 1) as usize;
    let actual_x = (max_el.0 - min_el.0 + 1) as usize;
    let mut bounds: Vec<Vec<u8>> = Vec::with_capacity(actual_y);
    let mut row: Vec<u8> = Vec::with_capacity(actual_x);
    row.resize(actual_x, 0);
    bounds.resize(actual_y, row);

    // shifting for the array
    let y_shift = min_el.1;
    let x_shift = min_el.0;

    //make bounds 
    for i in 1..data.len() + 1 {
        let mut start = data[i - 1].clone();
        let end = if i == data.len() { data[0] } else { data[i] };

        let delta = (start.0 - end.0, start.1 - end.1);
        let dx = if delta.0 == 0 { 0 } else if delta.0 < 0 { 1 } else { -1 };
        let dy = if delta.1 == 0 { 0 } else if delta.1 < 0 { 1 } else { -1 };
        for _ in 0..max(delta.0.abs(), delta.1.abs()) {
            start = (start.0 + dx, start.1 + dy);
            bounds[(start.1 - y_shift) as usize][(start.0 - x_shift) as usize] = 1;
        }
    }

    // okay make fun of me for allocating a ~100,000x~100,000 array and running a flood fill on
    // some big ass shape in the middle of it... but does a hashset really provide any less
    // overhead?? 
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..bounds[0].len() {
        if bounds[0][i] == 1 {
            if bounds[1][i] == 0 {
                q.push_back((i, 1));
                break;
            }
        }
    }

    const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    while !q.is_empty() {
        let current = q.pop_front().unwrap();

        if bounds[current.1][current.0] != 1 {
            bounds[current.1][current.0] = 1;

            for dir in DIRS {
                let new = ((current.0 as isize + dir.0) as usize, (current.1 as isize + dir.1) as usize);
                if bounds[new.1][new.0] == 0 {
                    q.push_back(new);
                }
            }
        }
    }

    // the point of this is to sort them by rectangles of the descending area, so that when we
    // actually find a valid rectangle, we can just exit
    let mut areas: BTreeMap<i64, ((i64, i64), (i64, i64))> = BTreeMap::new();
    for i in 0..data.len() {
        let c1 = data[i];
        for j in 0..data.len() {
            if i == j {
                continue;
            }

            let c2 = data[j];
            let area = ((c1.0 - c2.0).abs() + 1) * ((c1.1 - c2.1).abs() + 1);
            areas.insert(area, (c1, c2));
        }
    }

    // theres a lot of room for optimization here... maybe a binary search along the area values to
    // try and track down the largest valid rectangle faster? I dont think dividing the work evenly
    // is really gonna help.
    //
    // I'll likely come back and try to improve this later... I wanna finish the rest of the
    // puzzles first tho.
    let bounds = Arc::new(bounds);
    let areas = Arc::new(areas);
    let ranges = collect_ranges(areas.len(), 800);
    for range in ranges {

        let mut threads: Vec<thread::JoinHandle<i64>> = vec![];
        for i in range {
            let t_bounds = Arc::clone(&bounds);
            let t_areas = Arc::clone(&areas);
            let worker = thread::spawn(move || {
                let bounds = t_bounds;
                let (area, (c1, c2)) = t_areas.iter().rev().nth(i).unwrap();
                let dx = c1.0 - c2.0;
                let dy = c1.1 - c2.1;
                let dmy = if dy < 0 { 1 } else if dy == 0 { 0 } else { - 1 };
                let dmx = if dx < 0 { 1 } else if dx == 0 { 0 } else { - 1 };

                let mut pos = (c1.0, c1.1);

                for _ in 0..dy.abs() {
                    pos.1 += dmy;
                    for _ in 0..dx.abs() {
                        pos.0 += dmx;
                        if bounds[(pos.1 - y_shift) as usize][(pos.0 - x_shift) as usize] != 1 {
                            return -1;
                        }
                    }
                    pos.0 = c1.0;
                }

                return *area;
            });

            threads.push(worker);
        }

        for t in threads {
            let val = t.join().unwrap();
            if val > -1 {
                return val;
            }
        }
    }

    return -1;
}
