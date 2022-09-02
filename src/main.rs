use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("nums.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut count = -2;
    let mut prev_sum = 100000000;
    let mut first_num = 0;
    let mut second_num = 0;
    let mut third_num = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let sum = first_num + second_num + third_num;

        first_num = second_num;
        second_num = third_num;
        third_num = line.parse().unwrap();

        if  sum > prev_sum {
            count += 1;
        }


        prev_sum = sum;
    }
    println!("{}", count);
}
