use regex::Regex;

pub fn day12() {
    let data = util::get_data("data/day12.txt");
    let pat = Regex::new(r"^(\d+)x(\d+): ((?: *\d+)+)$").unwrap();
    let trees: Vec<((u64, u64), u64)> = data.iter()
        .filter(|l| pat.is_match(l))
        .map(|l| {
            let rx   = pat.captures(l).unwrap();

            let len  = rx.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let wid  = rx.get(2).unwrap().as_str().parse::<u64>().unwrap();
            let rest = rx.get(3).unwrap().as_str().split(' ');

            let rest: u64 = rest.map(|n| n.parse::<u64>().unwrap()).sum();
            ((len, wid), rest)
        })
        .collect();

    println!("p1: {}", part1(&trees));
}

// just put the boxes under the tree lol??
fn part1(trees: &Vec<((u64, u64), u64)>) -> u64 {
    trees.iter().map(|tree| {
        let ((len, wid), boxes) = tree;
        let area = (len / 3) * (wid / 3);
        if area >= *boxes { 1 } else { 0 }
    }).sum()
}
