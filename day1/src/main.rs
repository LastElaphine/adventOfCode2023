use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut calibration_sum = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let calibration = get_calibration(&line);
                calibration_sum += calibration;
                // println!("{} -> {}", line, calibration);
            }
        }
    }
    println!("Part1 -> {}", calibration_sum);
}

fn part2() {
    let mut calibration_sum = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let calibration = convert_line_to_numbers(&line);
                calibration_sum += calibration;
                // println!("{} -> {}", line, calibration);
            }
        }
    }
    println!("Part2 -> {}", calibration_sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn convert_line_to_numbers(line: &String) -> u32 {
    let numeric_map = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut first_number: u32 = 0;
    let mut first_index: usize = usize::MAX;
    for tuple in numeric_map {
        let (number_str, number_val) = tuple;
        line.find(number_str).map(|index| {
            if index <= first_index {
                first_index = index;
                first_number = number_val;
            }
        });
    }

    let mut last_number: u32 = 0;
    let mut last_index: usize = 0;
    for tuple in numeric_map {
        let (number_str, number_val) = tuple;
        line.rfind(number_str).map(|index| {
            if index >= last_index {
                last_index = index;
                last_number = number_val;
            }
        });
    }

    return first_number * 10 + last_number;
}

fn get_calibration(line: &String) -> u32 {
    let mut numbers = Vec::new();

    for letter in line.chars() {
        if letter.is_numeric() {
            // Map the result of to_digit since it retuns an optional
            letter.to_digit(10).map(|digit| numbers.push(digit));
        }
    }

    // Create the calibration value
    if !numbers.is_empty() {
        return numbers[0] * 10 + numbers[numbers.len() - 1];
    }

    return 0;
}