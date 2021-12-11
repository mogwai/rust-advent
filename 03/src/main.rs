use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
fn main() {
    // println!("Day 1");
    // println!("Challenge 1");
    ch1();
    // println!("Challenge 2");
    // ch2();
}

fn most_common(s: &String) -> i32 {
    let mut zero_count = 0;

    for c in s.chars() {
        if c == '0' {
            zero_count += 1;
        }
        println!("{}",c);
    }
    if zero_count > s.chars().count()/2 {
        0
    }
    1
}

// Use this to stop the warnings about the function not being used!
#[allow(dead_code)]
fn ch1() {
    // Create a path to the desired file
    let path = Path::new("./day3_input.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let lineLength = lines[0].len();
    let mut gamme_rate = String::new();
    let eps_rate = "";

    for i in 0..lineLength {
        let mut column = String::new();
        for line in &lines {
            let c = line.as_bytes()[i] as char;
            column.push(c);
        }
        println!("{}", column)
    }
    // println!("{}", lines[0].as_bytes()[0] as char);
    // let chars: Vec<Vec<String>> = [[]];
    // let chars = lines.map(|s| s.time().split(""));
    // dbg!(lines);
}

fn parse_float(line: &String) -> f32 {
    line.trim().parse::<f32>().expect("Failed to parse float")
}

#[allow(dead_code)]
fn ch2() {
    // Create a path to the desired file
    let path = Path::new("./day3_input.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();


    println!("{}", lines[0]);

}
