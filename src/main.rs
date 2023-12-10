use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let day = 1;
    let input_path = format!("input/{:02}.txt", day);
    let input = read_file_as_lines(&input_path).unwrap();
    let input = input.collect::<Result<Vec<String>, _>>().unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let sum: u32 = input
        .iter()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            let number = format!("{}{}", first, last).parse::<u32>().unwrap();
            number
        })
        .sum();
    println!("Sum: {}", sum);
}

fn part2(input: &Vec<String>) {
    let replacements = vec![
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let sum: u32 = input
        .iter()
        .map(|line| {
            let mut digits = line.clone();
            for (name, number) in replacements.iter() {
                digits = digits.replace(name, number);
            }
            digits = digits.chars().filter(|c| c.is_ascii_digit()).collect();
            let first = digits.chars().next().unwrap();
            let last = digits.chars().last().unwrap();
            let number = format!("{}{}", first, last).parse::<u32>().unwrap();
            // println!("{} -> {} -> {}", line, digits, number);
            number
        })
        .sum();
    println!("Sum: {}", sum);
}

fn read_file_as_lines(path: &str) -> Result<io::Lines<io::BufReader<File>>, io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines())
}
