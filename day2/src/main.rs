use std::fs::File;
use std::io::{self, BufRead};

fn verify(reports:&Vec<i32>) -> bool {

        let is_increasing = reports[0] < reports[1];
        if reports[0] == reports[1] {
            return false;
        }
        if (reports[0] - reports[1]).abs() > 3{
            return false;
        }
        for i in 1..reports.len()-1 {
            let new_is_increasing = reports[i] < reports[i+1];
            if is_increasing != new_is_increasing {
                return false;

            }
            if reports[i] == reports[i+1] {
                return false;

            }
            if (reports[i] - reports[i+1]).abs() > 3{
                return false;

            }
            
        }
        return true;
    
}

fn main() {
    let file = File::open("day2.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut input: Vec<Vec<i32>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        input.push(numbers);
    }
    let mut total = 0;
    // itterate over input 
    for reports in input.iter() {
        for i in 0..reports.len() {
            let mut new_vec = reports.clone();
            new_vec.remove(i);
            if verify(&new_vec){
                total+=1;
                break;
            }
        }        
        
    }
    println!("{}", total);
}
