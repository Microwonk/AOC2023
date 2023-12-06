use std::collections::HashMap;

use AOC2023::util::*;

fn first(inp: &Input) {
    let sum = inp
        .lines
        .iter()
        .map(|line| {
            let (winnings, ours) = cards(line);

            let mut count = 0;
            ours.iter().for_each(|num| {
                if winnings.contains(num) {
                    count += 1;
                }
            });

            match count {
                0 => 0,
                _ => (1..count).into_iter().fold(1, |acc, _| acc * 2)
            }
        })
        .sum::<u32>();

    println!("{}", sum);
}

fn second(inp: &Input) {

    let mut num_copies: HashMap<usize, u32> = HashMap::new();

    // extremely slow but idc
    inp
        .lines
        .iter()
        .enumerate()
        .for_each(|(i, line)| {
            for _ in 0..num_copies.get(&i).copied().unwrap_or(1) {
                let (winnings, ours) = cards(line);
                
                let mut count = 0;
                ours.iter().for_each(|num| {
                    if winnings.contains(num) {
                        count += 1;
                    }
                });

                ((i.saturating_sub(1))..=(count + i)).skip(1).for_each(|j| {
                    let new_val = num_copies.get(&j).copied().unwrap_or_default();
                    num_copies.insert(j, new_val + 1);
                });
            }
        });

    let sum = num_copies.values().sum::<u32>() + 1;

    println!("{sum}");
}

fn cards(line: &String) -> (Vec<&str>, Vec<&str>) {
    let cards = line.split(":").nth(1).unwrap().split_once("|").unwrap();

    let winnings: Vec<_> = cards.0
        .split_whitespace()
        .filter(|e| !e.is_empty())
        .collect();

    let ours: Vec<_> = cards.1
        .split_whitespace()
        .filter(|e| !e.is_empty())
        .collect();

    (winnings, ours)
}

fn main() {
    let mut inp = Input::new();
    inp.read("./res/4.txt").unwrap();
    first(&inp);
    second(&inp);
}
