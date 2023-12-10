use std::{collections::HashMap, fs};

fn main() {
    let contents = match fs::read_to_string("./puzzle-input.txt") {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    println!("part one: {}", part_one(contents.clone()));
    println!("part_two: {}", part_two(contents.clone()));
}

fn part_one(input_string: String) -> u32 {
    let mut sum = 0;

    'game: for line in input_string.lines() {
        let (game, sets) = line.split_once(": ").unwrap();
        let (_, game_id) = game.split_once(' ').unwrap();

        for set in sets.split("; ") {
            for cube in set.split(", ") {
                let (count, color) = cube.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                let is_valid = match color {
                    "red" => count <= 12,
                    "green" => count <= 13,
                    "blue" => count <= 14,
                    _ => true,
                };

                if !is_valid {
                    continue 'game;
                }
            }
        }

        sum += game_id.parse::<u32>().unwrap();
        println!("game: {}", game_id)
    }

    sum
}

fn part_two(input_string: String) -> u32 {
    let mut sum = 0;

    for line in input_string.lines() {
        let (_, sets) = line.split_once(": ").unwrap();

        let mut draws: HashMap<&str, u32> = HashMap::new();
        for set in sets.split("; ") {
            for cube in set.split(", ") {
                let (count, color) = cube.split_once(' ').unwrap();
                let count = count.parse::<u32>().unwrap();
                let val = draws.get(color);

                if val.is_none() {
                    draws.insert(color, count);
                }

                if count > *draws.get(color).unwrap() {
                    draws.insert(color, count);
                }
            }
        }
        let mut sub_total = 1;

        for (_, draw) in draws.iter() {
            sub_total *= draw;
        }

        sum += sub_total;
        // println!("sum: {:?}", sub_total)
    }

    sum
}
