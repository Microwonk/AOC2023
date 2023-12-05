use std::collections::HashSet;

use AOC2023::util::*;

fn first(inp: &Input) {
    let mut prev = inp.lines.first().unwrap();
    let mut curr = prev;
    let mut next = &inp.lines[1];

    let mut sum: u32 = 0;

    inp.lines.iter().enumerate().for_each(|(i, line)| {
        curr = line;
        next = if i + 1 < inp.lines.len() {
            &inp.lines[i + 1]
        } else {
            &inp.lines.last().unwrap()
        };

        let mut symb_indices: HashSet<usize> = HashSet::new();

        [prev, curr, next].iter().for_each(|l| {
            l.char_indices()
                .into_iter()
                .filter(|i| is_symbol(i.1))
                .for_each(|s| {
                    symb_indices.insert(s.0);
                });
        });

        let nums: Vec<_> = line
            .chars()
            .filter(|c| !is_symbol(*c))
            .into_iter()
            .collect::<String>()
            .split('.')
            .filter_map(|n| n.parse::<u32>().ok())
            .filter(|n_in_line| {
                symb_indices.iter().any(|s| {
                    match line.match_indices(&n_in_line.to_string()).next() {
                        Some(i) => !(*s < (i.0.saturating_sub(1)) || *s > i.0 + i.1.len()),
                        None => false,
                    }
                })
            })
            .collect();

        println!("{nums:?}");

        prev = line;
        sum += nums.iter().sum::<u32>();
    });

    println!("{sum}")
}

fn is_symbol(c: char) -> bool {
    !(c == '.' || c.is_numeric())
}

fn second(inp: &Input) {}

fn main() {
    let mut inp = Input::new();
    inp.read("./res/3.txt").unwrap();

    first(&inp);
    second(&inp);
}
