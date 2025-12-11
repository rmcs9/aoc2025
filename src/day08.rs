use disjoint::DisjointSetVec;

pub fn day08() {
    let data = util::get_data("data/day08.txt");
    let data: Vec<(i64, i64, i64)> = data.iter().map(|l| {
        let splt: Vec<i64> = l.split(',').map(|s| s.parse().unwrap()).collect();
        (splt[0], splt[1], splt[2])
    }).collect();

    let mut edges: Vec<((usize, usize), i64)> = Vec::new();

    // compute edges
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i == j {
                continue;
            }

            let dist = dist(&data[i], &data[j]);
            edges.push(((i, j), dist));
        }
    }
    
    // sort edges
    edges.sort_by_key(|e| e.1);
    // filter only one of each edge 
    let edges: Vec<((usize, usize), i64)> = edges.iter().enumerate().filter(|&(i, _)| i % 2 == 1).map(|(_, &e)| e).collect();

    println!("p1: {}", part1(&data, &edges));
    println!("p2: {}", part2(&data, &edges));
}

fn part1(data: &Vec<(i64, i64, i64)>, edges: &Vec<((usize, usize), i64)>) -> usize {
    let mut circuits: DisjointSetVec<(i64, i64, i64)> = DisjointSetVec::from(data.clone());
    for i in 0..data.len() {
        let edge = edges[i];
        circuits.join(edge.0.0 as usize, edge.0.1 as usize);
    }
    
    // obtain the interior circuits
    let mut circuits = circuits.indices().sets();
    // sort them ascending by size
    circuits.sort_by_key(|s| s.len());
    // reverse the sort, take the top 3 and return the product of their lengths
    return circuits.iter().rev().take(3).map(|s| s.len()).product();
}

fn part2(data: &Vec<(i64, i64, i64)>, edges: &Vec<((usize, usize), i64)>) -> usize {
    let mut circuits: DisjointSetVec<(i64, i64, i64)> = DisjointSetVec::from(data.clone());
    for i in 0..edges.len() {
        let edge = edges[i];
        circuits.join(edge.0.0 as usize, edge.0.1 as usize);

        // if this edge causes all the elements to merge into 1 set
        if circuits.indices().sets().len() == 1 {
            return (data[edge.0.0].0 * data[edge.0.1].0) as usize; 
        }
    }
    0
}

fn dist(p1: &(i64, i64, i64), p2: &(i64, i64, i64)) -> i64 {
    let t1 = (p1.0 - p2.0).pow(2);
    let t2 = (p1.1 - p2.1).pow(2);
    let t3 = (p1.2 - p2.2).pow(2);

    t1 + t2 + t3
}
