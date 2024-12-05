use std::fs::{File};
use std::io::{self, BufRead};

fn main() {
    let file = File::open("day2.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        arr1.push(numbers[0]);
        arr2.push(numbers[1]);
    }
    arr1.sort();
    arr2.sort();
    let mut total = 0;
    let mut i = 0;
    while i < arr1.len() {
        let mut count = 0; 
        for j in 0..arr2.len() {
            if arr1[i] == arr2[j] {
                count += 1;
            }
        }
        total += arr1[i] * count;
        i+=1;

    }
    println!("{}", total);
    
}
