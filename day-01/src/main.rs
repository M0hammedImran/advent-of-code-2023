use std::fs;

fn main() {
    let contents = match fs::read_to_string("./puzzle-input.txt") {
        Ok(data) => data,
        Err(err) => panic!("{}", err),
    };

    let val = contents
        .split('\n')
        .map(|line| {
            return line
                .split("")
                .filter_map(|number_or_str| number_or_str.parse::<i32>().ok())
                .collect::<Vec<_>>();
        })
        .map(|numbers| {
            return (numbers.first().unwrap().to_string() + &numbers.last().unwrap().to_string())
                .parse::<i32>()
                .unwrap();
        })
        .sum::<i32>();

    println!("{:?}", val);
}
