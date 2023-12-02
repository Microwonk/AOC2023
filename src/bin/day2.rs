use AOC2023::util::*;

const COLORS: [(u32, &str); 3] = [(12, "red"), (13, "green"), (14, "blue")];

fn first(inp: Input) {
    let res: u32 = inp.data.iter().filter_map(|game| {
        let parts: Vec<&str> = game.split(":").collect();

        if let [id_part, data_part] = parts.as_slice() {
            let id = id_part.split_whitespace().nth(1).unwrap_or_default().parse::<u32>();

            let is_game_possible = data_part.split(";").all(|g| {
                !g.split(",").any(|r| {
                    let mut part = r.split_whitespace();
                    let num_color = part.nth(0).unwrap_or_default().parse::<u32>();

                    num_color.map_or(false, |val| {
                        COLORS.iter().any(|(limit, color)| val > *limit && r.contains(color))
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

fn second(inp: Input) {
    let res: u32 = inp.data.iter().filter_map(|game| {
        let parts: Vec<&str> = game.split(":").collect();

        if let [_, data_part] = parts.as_slice() {

            let mut min_r = 1;
            let mut min_g = 1;
            let mut min_b = 1;

            data_part.split(";").for_each(|g| {
                g.split(",").for_each(|r| {
                    let mut part = r.split_whitespace();
                    let num_color = part.nth(0).unwrap_or_default().parse::<u32>().unwrap_or_default();

                    match r {
                        _ if r.contains("red") => min_r = min_r.max(num_color),
                        _ if r.contains("green") => min_g = min_g.max(num_color),
                        _ if r.contains("blue") => min_b = min_b.max(num_color),
                        _ => {}
                    }
                });
            });

            Some(min_r * min_g * min_b)
        } else {
            None
        }
    }).sum();

    println!("{}", res);
}

fn main() {
    let mut inp = Input::new();
    inp.read("./res/2.txt").unwrap();
    first(inp.clone());
    second(inp.clone());
}
