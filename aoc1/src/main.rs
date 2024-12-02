use std::{fs::File, io::BufReader, io::BufRead};

fn main() {

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();
    let file = File::open("input.txt").expect("Unable to open input.txt");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse().unwrap());
        right.push(split.next().unwrap().parse().unwrap());
        //println!("{} {}", left.last().unwrap(), right.last().unwrap());
        
    }
    left.sort();
    right.sort();

    let mut count: u64 = 0;

    for l_value in left.iter() {
        let mut match_count: u32 = 0;
        for r_value in right.iter() {
            if l_value == r_value {
                println!("{} {}", l_value, r_value);
                match_count += 1;
            }
        }
        count += (l_value * match_count) as u64;
    }
    println!("Total similarity: {count}");

}
