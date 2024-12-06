use std::fs::File;
use std::io::{self, BufRead, Read};
use regex::Regex;


fn main() {
    let mut file = File::open("day3.txt").unwrap();
    let mut total = 0;

    let mut enabled = true;
    let mut nums_pos: Vec<u64> = Vec::new();
    let mut nums_val: Vec<u64> = Vec::new();
    let mut enable_pos: Vec<u64> = Vec::new();
    let mut disable_pos: Vec<u64> = Vec::new();

    let mut input: String = String::new();
    file.read_to_string(&mut input).unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    

    for mat in re.find_iter(&input) {
        nums_pos.push(mat.start() as u64);
        if let Some(caps) = re.captures(&mat.as_str()) {
            nums_val.push( caps.get(1).unwrap().as_str().parse::<u64>().unwrap() * caps.get(2).unwrap().as_str().parse::<u64>().unwrap());
        }
    }
    
    let re_do = Regex::new(r"do\(\)").unwrap(); 
    for mat in re_do.find_iter(&input) {
        enable_pos.push(mat.start() as u64);
    }
    
    let re_dont = Regex::new(r"don't\(\)").unwrap(); 
    for mat in re_dont.find_iter(&input) {
        disable_pos.push(mat.start() as u64);
    }

        





    

    let mut pos = 0;
    let mut en_index = 0;
    let mut dis_index = 0;

    for i in 0..nums_pos[nums_pos.len()-1]+1 {
        
        if en_index < enable_pos.len() && i == enable_pos[en_index]{
            println!("enabled {}", &input[enable_pos[en_index] as usize..enable_pos[en_index] as usize + 12]);
            enabled = true;
            en_index+=1;
        }
        if dis_index < disable_pos.len() && i == disable_pos[dis_index]{
            println!("disabled {}", &input[disable_pos[dis_index] as usize..disable_pos[dis_index] as usize + 12]);
            enabled = false;
            dis_index+=1;
        }
        if pos < nums_pos.len() && i == nums_pos[pos] {
            if enabled {
                println!("added {} {}", nums_val[pos], &input[nums_pos[pos] as usize..nums_pos[pos] as usize + 12]);
                total += nums_val[pos];
            }else{
                println!("skipped");

            }
            pos += 1;
        }
        

    }
    // itterate over input 
    println!("{}", total);
   
}
