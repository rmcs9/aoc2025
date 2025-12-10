use std::env;

mod day01;
mod day02;
mod day03; 
mod day04;
mod day05;
mod day06;
mod day07;

const FUNCTIONS: [fn(); 7] = [day01::day01, day02::day02, day03::day03, day04::day04, day05::day05, day06::day06, day07::day07];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        run_all();   
    } else {
        if args.len() > 2  {
            panic!("invalid argument... example format: cargo run -- [1..12]");
        }

        let day = &args[1].parse::<usize>();
        match day {
            Ok(i) => { FUNCTIONS[i - 1]() }, 
            Err(..) => { panic!("invalid argument... example format: cargo run -- [1..12]") },
        }
    }
}

fn run_all() {
    for i in 0..FUNCTIONS.len() {
        println!("--------------------");
        println!("DAY {}", i + 1); 
        FUNCTIONS[i]();
    }
    println!("--------------------");
}
