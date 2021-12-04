use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
fn main() {
    // println!("Day 1");
    // println!("Challenge 1");
    // ch1();
    // println!("Challenge 2");
    ch2();
}

// Use this to stop the warnings about the function not being used!
#[allow(dead_code)]
fn ch1() {
    // Create a path to the desired file
    let path = Path::new("/home/h/input.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut lines = reader.lines().enumerate();
    // firstline = firstline.
    let mut prev: i32 = lines
        .next()
        .unwrap()
        .1
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();
    let mut increased = 0;

    for (_index, line) in lines {
        let lineint: i32 = line.unwrap().trim().parse::<i32>().unwrap();
        // Debugging:
        // dbg!(prev);
        // dbg!(lineint);
        // dbg!(increased);
        if prev < lineint {
            increased += 1;
        }
        prev = lineint;
    }
    println!("{}", increased)
}

fn parse_float(line: &String) -> f32 {
    line.trim().parse::<f32>().expect("Failed to parse float")
}

#[allow(dead_code)]
fn ch2() {
    // Create a path to the desired file
    let path = Path::new("./day1_input.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    println!("{}", lines[0]);
    let mut increased = 0;
    for i in 0..lines.len() - 3 {
        let mut mov1: f32 = 0.;
        for j in 0..3 {
            // dbg!(j);
            mov1 += parse_float(&lines[i + j]);
        }
        println!(
            "mov1=[{},{},{}]={}",
            lines[i + 0],
            lines[i + 1],
            lines[i + 2],
            mov1
        );

        let mut mov2: f32 = 0.;
        for j in 1..4 {
            // dbg!(j);
            mov2 += parse_float(&lines[i + j]);
        }
        println!(
            "mov2=[{},{},{}]={}",
            lines[i + 1],
            lines[i + 2],
            lines[i + 3]
            mov2
        );
        if mov2 > mov1 {
            increased += 1;
        }
        println!("{}", mov2 > mov1);
    }

    println!("{}", increased)
}
