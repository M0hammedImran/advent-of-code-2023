use std::{collections::HashMap, fs};

fn main() {
    let contents = match fs::read_to_string("./puzzle-input.txt") {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    println!("part one: {}", part_one(contents.clone()));
    println!("part two: {}", part_two(contents.clone()));
}

fn part_one(input_string: String) -> i32 {
    return input_string
        .split('\n')
        .map(|line| {
            return line
                .split("")
                .filter_map(|number_or_str| number_or_str.parse::<i32>().ok())
                .collect::<Vec<_>>();
        })
        .map(|numbers| {
            if numbers.is_empty() {
                return 0;
            }

            return (numbers.first().unwrap().to_string() + &numbers.last().unwrap().to_string())
                .parse::<i32>()
                .unwrap();
        })
        .sum::<i32>();
}
fn part_two(input_string: String) -> u32 {
    let digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    return input_string
        .split('\n')
        .map(|line| {
            let mut line = line.to_string();
            let mut line_indexes = Vec::new();

            for (digit, _) in digits.clone().into_iter() {
                let index = line.find(digit);
                if let Some(index) = index {
                    line_indexes.push(index.to_string() + "_" + digit);
                }
            }

            line_indexes.sort();
            line_indexes.clone().into_iter().for_each(|line_index| {
                let v_digit = line_index.split('_').collect::<Vec<_>>()[1];
                line = line.replace(
                    v_digit,
                    &(digits.get(v_digit).unwrap().to_string() + v_digit),
                );
            });

            return line
                .chars()
                .filter_map(|number_or_str| number_or_str.to_digit(10))
                .collect::<Vec<_>>();
        })
        .map(|numbers| {
            let sum = (10 * numbers.first().unwrap()) + numbers.last().unwrap();
            sum
        })
        .sum();
}
