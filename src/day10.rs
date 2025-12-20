use regex::Regex;
use std::collections::HashSet;

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

fn part2(machines: &Vec<Machine>) -> u64 {
    0
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
