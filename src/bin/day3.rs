use std::collections::HashSet;

use AOC2023::util::*;

fn first(inp: &Input) {
    let mut prev = inp.lines.first().unwrap();
    let mut next: &String = &"".to_string();

    let mut sum: u32 = 0;

    inp.lines.iter().enumerate().for_each(|(i, line)| {
        next = if i + 1 < inp.lines.len() {
            &inp.lines[i + 1]
        } else {
            &inp.lines.last().unwrap()
        };

        let symb_indices: HashSet<_> = [prev, line, next]
            .iter()
            .flat_map(|l| l.char_indices().filter(|i| is_symbol(i.1)).map(|s| s.0))
            .collect();

        println!("ind {symb_indices:?}");

        let nums: Vec<_> = line
            .split('.')
            .filter_map(|n| {
                n.chars()
                    .filter(|c| !is_symbol(*c))
                    .collect::<String>()
                    .parse::<u32>()
                    .ok()
            })
            .filter(|n_in_line| {
                symb_indices.iter().any(|s| {
                    match line.match_indices(&n_in_line.to_string()).next() {
                        Some(i) => (i.0.saturating_sub(1)..=(i.0 + i.1.len() + 1)).contains(s),
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
    c != '.' && !c.is_numeric()
}

fn second(inp: &Input) {
    // TODO
}

fn main() {
    let mut inp = Input::new();
    inp.read("./res/3.txt").unwrap();

    first(&inp);
    second(&inp);
}
