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
            println!("line: {:?}", line);

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
                line = line.replace(v_digit, digits.get(v_digit).unwrap().to_string().as_str());
            });

            return line
                .chars()
                .filter_map(|number_or_str| number_or_str.to_digit(10))
                .collect::<Vec<_>>();
        })
        .map(|numbers| {
            println!("numbers: {:?}", numbers);
            let sum = (10 * numbers.first().unwrap()) + numbers.last().unwrap();
            sum
        })
        .sum();
}

fn parse_input(input: &str, replace: bool) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line = line.to_string();
            println!("line: {:?}", line);
            if replace {
                line = line
                    .to_string()
                    .replace("one", "one1one")
                    .replace("two", "two2two")
                    .replace("three", "three3three")
                    .replace("four", "four4four")
                    .replace("five", "five5five")
                    .replace("six", "six6six")
                    .replace("seven", "seven7seven")
                    .replace("eight", "eight8eight")
                    .replace("nine", "nine9nine");
            }
            line
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| {
            println!("vec: {:?}", vec);
            10 * vec.first().unwrap() + vec.last().unwrap()
        })
        .sum()
}
