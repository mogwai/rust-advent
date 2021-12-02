use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("Part 1");
    part1();
    println!("Part 2");
    part2();
}

fn part1() {
    // Create a path to the desired file
    let path = Path::new("/home/h/advent_of_code/day2/2input.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let lines = reader.lines().enumerate();
    let mut depth = 0;
    let mut horizontal = 0;

    for (_index, line) in lines {
        let instruction = line.unwrap();
        // let instruc = instruction.split(" ");
        let split: Vec<&str> = instruction.split(" ").collect();

        let movement = split[0];
        let amount: i32 = split[1].parse().unwrap();
        match movement {
            "forward" => horizontal += amount,
            "up" => depth -= amount,
            "down" => depth += amount,
            &_ => println!("Failed"),
        }
    }
    println!("depth {} horizontal {}", depth, horizontal);
    println!("depth x horizontal = {}", depth * horizontal);
}

fn part2() {
    // Create a path to the desired file
    let path = Path::new("/home/h/advent_of_code/day2/2input.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let lines = reader.lines().enumerate();
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for (_index, line) in lines {
        let instruction = line.unwrap();
        // let instruc = instruction.split(" ");
        let split: Vec<&str> = instruction.split(" ").collect();

        let movement = split[0];
        let amount: i32 = split[1].parse().unwrap();
        match movement {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            },
            "up" => {
                aim -= amount;
            },
            "down" => {
                aim += amount;
            },
            &_ => println!("Failed"),
        }
    }
    println!("depth {} horizontal {}", depth, horizontal);
    println!("depth x horizontal = {}", depth * horizontal);
}
