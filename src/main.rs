use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

fn two_sum(nums: Vec<i128>, target: i128) -> Vec<i128> {
    let mut seen = std::collections::HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        if let Some(p) = seen.get(n) {
            return vec![*p, i as i128];
        }
        seen.insert(target - *n, i as i128);
    }
    vec![]
}

fn sum(vector: Vec<i128>) -> i128 {
    for i in 100..vector.len() - 100 {
        if two_sum(vector[i - 100..i].to_vec(), vector[i] as i128).is_empty() {
            return vector[i];
        }
    }
    return 0;
}

fn main() {
    let file = File::open("./src/challenge_input.txt").expect("Error opening the file");
    let br = BufReader::new(file);
    let mut v: Vec<i128> = vec![];
    for line in br.lines() {
        v.push(
            line.unwrap()
                .trim()
                .parse()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))
                .unwrap(),
        );
    }
    let solution = sum(v);
    println!("The first unsafe number is {}", solution);
}
