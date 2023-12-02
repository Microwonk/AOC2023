use AOC2023::util::*;
use std::collections::BTreeMap;

pub const NUMERICAL_WORDS: [&str; 9] = [
    "one",
    "two", 
    "three",
    "four",
    "five",
    "six", 
    "seven",
    "eight",
    "nine",
    ];

#[allow(dead_code)]
pub fn first() {
    let mut inp = Input::new();
    inp.read("./res/1.txt").unwrap();

    let calibration_values: u32 = inp.data.iter().map(|l| {
        let digits: Vec<u32> = l.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        match digits.len() {
            0 => 0, // never the case, but still
            _ => digits[0] * 10 + digits.last().unwrap(),
        }
    }).sum();

    println!("Calibration Value: {}", calibration_values);
}

pub fn second() {
    let mut inp = Input::new();
    inp.read("./res/1.txt").unwrap();

    let calibration_values: u32 = inp.data.iter().map(|l| {
        let mut digit_positions = BTreeMap::new();

        NUMERICAL_WORDS.iter().enumerate().for_each(|(i, &digit)| {
            for (pos, _) in l.match_indices(digit) {
                digit_positions.insert(pos, (i + 1) as u8);
            }
        });

        l.chars().enumerate().for_each(|(pos, ch)| {
            if ch.is_numeric() {
                digit_positions.insert(pos, ch as u8 - b'0');
            }
        });

        let digits: Vec<u8> = digit_positions.values().cloned().collect();
        u32::from_str_radix(&format!("{}{}", digits[0], digits.last().unwrap()), 10).unwrap_or(0)
    }).sum();

    println!("Calibration Values: {}", calibration_values);
}


fn main() {
    first();
    second();
}

