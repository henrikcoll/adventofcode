use std::fs::{File, read_to_string};
use std::io::Write;

fn parse_number(s: &str) -> Option<i32> {
    match s.parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn replace_words(s: &str) -> String {
    s.replace("zero", "z0o")
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn get_calibration_value(s: &str) -> i32 {
    let numbers: Vec<i32> = s
            .split("")
            .filter_map(|num| parse_number(num))
            .collect();
        
    let first = numbers[0];
    let last = numbers[numbers.len() - 1];
    return first * 10 + last;
}

fn part_a(filename: &str) -> i32 {
    let mut calibration_values = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let calibration_value = get_calibration_value(&line);
        calibration_values.push(calibration_value);
    }

    calibration_values.iter().sum()
}

fn part_b(filename: &str) -> i32 {
    let mut calibration_values = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let line2 = replace_words(&line);
        let calibration_value = get_calibration_value(&line2);
        calibration_values.push(calibration_value);
    }

    calibration_values.iter().sum()
}

fn main() {
    let answer_a = part_a("input.txt");
    let answer_b = part_b("input.txt");


    let mut file_a = File::create("answer_a.txt").unwrap();
    write!(file_a, "{}", answer_a).unwrap();

    let mut file_b = File::create("answer_b.txt").unwrap();
    write!(file_b, "{}", answer_b).unwrap();
}

