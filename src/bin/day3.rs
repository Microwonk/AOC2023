use core::num;

use AOC2023::util::*;

fn first(inp: &Input) {

    let mut prev: &str = ".";
    let mut next: &str = ".";

    let mut sum: u32 = 0;

    for (i, line) in inp.lines.iter().enumerate()  {

        let nums: Vec<u32> = line.split(".").filter_map(|e| {
            if !e.is_empty() {
                if let Ok(num) = e.parse::<u32>() {
                    return Some(num);
                }
            }
            None
        }).collect();

        nums.iter().for_each(|num| {
            println!("{:?}", [prev, line, next]);
            if i != 0 {
                if match_lines([prev, line, next], *num, i) {
                    sum += num;
                }
            }
            
        });

        prev = line;
    }

    println!("{}", sum);
}

fn match_lines(inputs: [&str; 3], number: u32, line: usize) -> bool {

    let num_index = inputs[line].match_indices(number.to_string().as_str()).collect::<Vec<_>>()[0];

    if num_index. {
        
    }

    inputs.iter().any(|line| {
        (num_index.0.saturating_sub(1)..(num_index.0 + num_index.1.len() + 1).min(inputs[0].len())).any(|range| {
            line.chars().collect::<Vec<_>>()[range] != '.'
        })
    })
}

fn second(inp: &Input) {

}

fn main() {
    let mut inp = Input::new();
    inp.read("./res/3.txt").unwrap();

    first(&inp);
    second(&inp);
}