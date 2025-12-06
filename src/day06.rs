
pub fn day06() {
    let data = util::get_data("data/day06.txt");
    let mut filtered: Vec<Vec<&str>> = Vec::new();
    for line in &data {
        let splt: Vec<&str> = line.split(' ').filter(|s| *s != "").collect();      
        filtered.push(splt);
    }

    println!("p1: {}", part1(&filtered));
    println!("p2: {}", part2(&data));
}

fn part1(data: &Vec<Vec<&str>>) -> u64 {
    let mut result = 0;
    let line_length = data[0].len();
    let data_length = data.len();
    for i in 0..line_length {
        let sign = data[data_length - 1][i];
        let mut op_total = match sign {
            "*" => 1, 
            "+" => 0, 
            _ => { panic!("invalid sign: {sign}"); }
        };
        for j in 0..data_length - 1 {
            let num: u64 = data[j][i].parse().unwrap();
            op_total = match sign {
                "*" => op_total * num,
                "+" => op_total + num, 
                _ => { panic!("invalid sign: {sign}"); }
            }
        }
        result += op_total;
    }
    return result;
}

fn part2(data: &Vec<String>) -> u64 {
    let new_data: Vec<Vec<char>> = data.iter().map(|l| l.chars().collect()).collect();
    return slice_n_calc(&new_data, 0);
}

// this is most certainly the worst rust code I have written thus far...
// this most likely goes against everything the core rust philosophy stands for 
fn slice_n_calc(data: &Vec<Vec<char>>, start_index: usize) -> u64 {
    let mut new_start: usize = data[0].len();
    let sign = data[data.len() - 1][start_index];
    let mut factors: Vec<u64> = Vec::new();

    // push all of the factors into a vector
    for i in start_index..data[0].len() {
        let mut str = String::new();
        for j in 0..data.len() - 1 {
            let c = data[j][i];
            if c != ' ' {
                str.push(data[j][i]);    
            }
        }

        if str.is_empty() {
            new_start = i + 1;
            break;
        }
        factors.push(str.parse().unwrap());
    }

    let mut op_total = match sign {
        '*' => 1, 
        '+' => 0, 
        _ => { panic!("invalid sign: {sign}"); }
    };

    // evaluate the expression
    for factor in factors {
        op_total = match sign {
            '*' => op_total * factor,
            '+' => op_total + factor, 
            _ => { panic!("invalid sign: {sign}"); }
        }
    }

    if new_start < data[0].len() {
        return op_total + slice_n_calc(data, new_start);
    }
    return op_total;
}
