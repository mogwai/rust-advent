use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

fn main() {
    // Create a path to the desired file
    let path = Path::new("/home/h/input.txt");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let mut lines = reader.lines().enumerate();
    // firstline = firstline.
    let mut prev: i32 = lines.next().unwrap().1.unwrap().trim().parse::<i32>().unwrap();
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
