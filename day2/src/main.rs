use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("nums.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let split = line.split(" ");
        let vec = split.collect::<Vec<&str>>();
        // println!("{}", vec[0]);
        let my_string = vec[1].to_string();
        let my_int = my_string.parse::<i32>().unwrap();
        if vec[0] == "forward"{
            pos += my_int;
            depth += aim * my_int;
        } else if vec[0] == "down" {
            aim += my_int;
        } else if vec[0] == "up" {
            aim -= my_int;
        }
    }

    println!("DEPTH: {} POS: {}", depth, pos);
    println!("MULTIPLIED: {}", depth * pos);
}
