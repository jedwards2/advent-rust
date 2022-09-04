use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("nums.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut array: [i32; 5] = [0; 5];

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let split = line.split("");
        let vec = split.collect::<Vec<&str>>();
        let mut i = 0;
        for (s, e) in vec.iter().enumerate() {
            println!("Element at position {}: {:?}", s, e);
            if s == "1" {
                println!("Hi");
                array[i] += 1;
            }
            i += 1;
        }
    }
    println!("{:?}", array);
}
