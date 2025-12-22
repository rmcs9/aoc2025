use regex::Regex;
use std::collections::HashSet;
use z3::{Config, Context, Optimize, SatResult, ast::Int, ast::Ast};

pub fn day10() {
    let data = util::get_data("data/day10.txt");
    let mut machines: Vec<Machine> = Vec::new();

    for line in data {
        machines.push(make_machine(&line));
    }

    println!("p1: {}", part1(&machines));
    println!("p2: {}", part2(&machines));
}

fn part1(machines: &Vec<Machine>) -> u64 {
    let mut presses = 0;
    'machines: for machine in machines {
        let mut light_states: HashSet<u32> = HashSet::new();
        light_states.insert(0);
        let mut machine_presses = 1;
        loop {
            let mut new_light_states: HashSet<u32> = HashSet::new();

            for state in &light_states {
                for button in &machine.buttons {
                    let new_state = state ^ button;   
                    if new_state == machine.lights {
                        presses += machine_presses;
                        continue 'machines;
                    } else {
                        new_light_states.insert(new_state);
                    }
                }
            }

            light_states = new_light_states;
            machine_presses += 1;
        }
    }
    presses
}

// generic z3 solution
fn part2(machines: &Vec<Machine>) -> u64 {
    let mut total_presses: u64 = 0;
    for machine in machines {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let opt = Optimize::new(&ctx);

        let num_buttons = machine.buttons.len();
        let num_lights = machine.joltage.len();

        let presses: Vec<Int> = (0..num_buttons)
            .map(|i| Int::new_const(&ctx, format!("p{}", i)))
            .collect();

        for p in &presses {
            opt.assert(&p.ge(&Int::from_i64(&ctx, 0)));
        }

        for light_idx in 0..num_lights {
            let mut terms: Vec<Int> = Vec::new();

            for (button_idx, button_mask) in machine.buttons.iter().enumerate() {
                if (button_mask & (1 << light_idx)) != 0 {
                    terms.push(presses[button_idx].clone());
                }
            }

            let sum = if terms.is_empty() {
                Int::from_i64(&ctx, 0)
            } else {
                Int::add(&ctx, &terms.iter().collect::<Vec<_>>())
            };

            opt.assert(&sum._eq(&Int::from_u64(&ctx, machine.joltage[light_idx] as u64)));
        }

        let total = Int::add(&ctx, &presses.iter().collect::<Vec<_>>());
        opt.minimize(&total);

        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt.get_model().unwrap();
                let machine_presses: i64 = model
                    .eval(&total, true)
                    .unwrap()
                    .as_i64()
                    .unwrap();
                total_presses += machine_presses as u64;
            }
            _ => panic!("No solution found for machine"),
        }
    }
    return total_presses
}


struct Machine {
    lights: u32, 
    buttons: Vec<u32>, 
    joltage: Vec<u32>,
}

fn make_machine(line: &String) -> Machine {
    let machine_pattern = Regex::new(r"^\[([.#]+)\]((?: \([\d,]+\))+) \{([\d,]+)\}$").unwrap();
    let rx = machine_pattern.captures(line).unwrap();

    let lights  = make_lights(rx.get(1).unwrap().as_str());
    let buttons = make_buttons(rx.get(2).unwrap().as_str());
    let joltage = make_joltage(rx.get(3).unwrap().as_str());

    let this_machine: Machine = Machine {
        lights: lights, 
        buttons: buttons,
        joltage: joltage,
    };
    this_machine
}

fn make_lights(light_str: &str) -> u32 {
    let mut light_int: u32 = 0;
    let mut i = 0;
    for bit in light_str.chars() {
        if bit == '#' {
            let mask: u32 = 1 << i;
            light_int = light_int | mask;
        }
        i += 1;
    }
    light_int
}

fn make_buttons(button_str: &str) -> Vec<u32> {
    let mut buttons: Vec<u32> = Vec::new();
    let button_str = button_str.trim();

    for splt in button_str.split(' ') {
        let splt = splt.strip_prefix("(").unwrap();
        let splt = splt.strip_suffix(")").unwrap();
        let nums: Vec<u32> = splt.split(',').map(|s| s.parse().unwrap()).collect();

        let mut button: u32 = 0;
        for num in nums {
            let mask: u32 = 1 << num;
            button = button | mask;
        }
        buttons.push(button);
    }
    buttons
}

fn make_joltage(jolt_str: &str) -> Vec<u32> {
    let jolt_str = jolt_str.trim();
    let splt = jolt_str.split(',');
    splt.map(|s| s.parse::<u32>().unwrap()).collect()
}
