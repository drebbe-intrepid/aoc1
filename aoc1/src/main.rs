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

    for (i, l_value) in left.iter().enumerate() {
        let r_value = right[i];
        println!("{} {}", l_value, r_value);
        count += l_value.abs_diff(r_value) as u64;
    }


    println!("Total Distance: {count}");
}
