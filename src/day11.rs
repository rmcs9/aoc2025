use std::collections::HashMap;
use regex::Regex;

pub fn day11() {
    let data = util::get_data("data/day11.txt");
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    let pat = Regex::new(r"^([a-z]{3}):((?: [a-z]{3})+)$").unwrap();
    data.iter().for_each(|line| {
        let rx = pat.captures(line).unwrap();  
        let key = rx.get(1).unwrap().as_str();
        let vals: Vec<&str> = rx.get(2).unwrap().as_str().split(' ').filter(|w| *w != "" ).collect();
        graph.insert(key, vals);
    });

    let mut mem = HashMap::new();
    println!("p1: {}", part1(&graph, "you"));
    println!("p2: {}", part2(&graph, &mut mem, ("svr", false, false)));
}

fn part1(graph: &HashMap<&str, Vec<&str>>, start: &str) -> u64 {
    if start == "out" {
        return 1;
    }
    graph.get(start).unwrap().iter()
        .map(|node| part1(&graph, node))
        .sum()
}

fn part2(graph: &HashMap<&str, Vec<&str>>, mem: &mut HashMap<(String, bool, bool), u64>, state: (&str, bool, bool)) -> u64 {
    let (start, dac, fft) = state;
    if start == "out" {
        return if dac && fft { 1 } else { 0 };
    }
    let k = (String::from(start), dac, fft);
    if mem.contains_key(&k) {
        return *mem.get(&k).unwrap();
    }
    let dac = if start == "dac" { true } else { dac };
    let fft = if start == "fft" { true } else { fft };
    let total = graph.get(start).unwrap().iter()
        .map(|node| part2(&graph, mem, (node, dac, fft)))
        .sum();
    mem.insert(k, total);
    return total;
}
