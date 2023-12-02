use AOC2023::util::*;

const COLORS: [(u32, &str); 3] = [(12, "red"), (13, "green"), (14, "blue")];

fn first(inp: &Input) {
    let res: u32 = inp.lines.iter().filter_map(|game| {
        let parts: Vec<&str> = game.split(":").collect();

        if let [id_part, data_part] = parts.as_slice() {
            let id = id_part.split_whitespace().nth(1).unwrap_or_default().parse::<u32>();

            let is_game_possible = data_part.split(";").all(|g| {
                !g.split(",").any(|round| {
                    let mut part = round.split_whitespace();
                    let num_color = part.nth(0).unwrap_or_default().parse::<u32>();

                    num_color.map_or(false, |val| {
                        COLORS.iter().any(|(limit, color)| val > *limit && round.contains(color))
                    })
                })
            });

            id.ok().filter(|_| is_game_possible)
        } else {
            None
        }
    }).sum();

    println!("{}", res);
}

fn second(inp: &Input) {
    let res: u32 = inp.lines.iter().filter_map(|game| {
        // id not needed
        let data_part: &str = game.split(":").nth(1).unwrap();

        let mut min_r = 1;
        let mut min_g = 1;
        let mut min_b = 1;

        data_part.split(";").for_each(|g| {
            g.split(",").for_each(|round| {
                let mut part = round.split_whitespace();
                let num_color = part.nth(0).unwrap().parse::<u32>().unwrap_or_default();

                match round {
                    _ if round.contains("red") => min_r = min_r.max(num_color),
                    _ if round.contains("green") => min_g = min_g.max(num_color),
                    _ if round.contains("blue") => min_b = min_b.max(num_color),
                    _ => {}
                }
            });
        });
        Some(min_r * min_g * min_b)
    }).sum();

    println!("{}", res);
}

fn main() {
    let mut inp = Input::new();
    inp.read("./res/2.txt").unwrap();
    first(&inp);
    second(&inp);
}
