use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut total = 0;

    let mut data: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() { 
        let line = line.unwrap();
        let mut chars: Vec<char> = Vec::new();
        for c in line.chars() {
            chars.push(c);
        }
        if chars.len() > 0 {
            
            data.push(chars);
        }
    }



    // for x in 0..data[0].len()-3 {
    //     for y in 0..data.len() {
    //         if (data[y][x] == 'X' && data[y][x+1] == 'M' && data[y][x+2] == 'A' && data[y][x+3] == 'S'){ // top to bottom
    //             total += 1;
    //         }
    //         if (data[y][x] == 'S' && data[y][x+1] == 'A' && data[y][x+2] == 'M' && data[y][x+3] == 'X'){ // bottom to top
    //             total += 1;
    //         }
    //     }
    // }




    // for x in 0..data[0].len() {
    //     for y in 0..data.len()-3 {
    //         if (data[y][x] == 'X' && data[y+1][x] == 'M' && data[y+2][x] == 'A' && data[y+3][x] == 'S'){ // top to bottom
    //             total += 1;
    //         }
    //         if (data[y][x] == 'S' && data[y+1][x] == 'A' && data[y+2][x] == 'M' && data[y+3][x] == 'X'){ // bottom to top
    //             total += 1;
    //         }
    //     }
    // }

    for x in 0..data[0].len()-2 {
        for y in 0..data.len()-2 {
            if (data[y][x] == 'M' && data[y+1][x+1] == 'A' && data[y+2][x+2] == 'S' && data[y+2][x] == 'M' && data[y+1][x+1] == 'A' && data[y][x+2] == 'S'){
                total += 1;
            }
            if (data[y][x] == 'S' && data[y+1][x+1] == 'A' && data[y+2][x+2] == 'M' && data[y+2][x] == 'S' && data[y+1][x+1] == 'A' && data[y][x+2] == 'M'){
                total += 1;
            }
            if (data[y][x] == 'M' && data[y+1][x+1] == 'A' && data[y+2][x+2] == 'S' && data[y+2][x] == 'S' && data[y+1][x+1] == 'A' && data[y][x+2] == 'M'){
                total += 1;
            }
            if (data[y][x] == 'S' && data[y+1][x+1] == 'A' && data[y+2][x+2] == 'M' && data[y+2][x] == 'M' && data[y+1][x+1] == 'A' && data[y][x+2] == 'S'){
                total += 1;
            }
        }
    }

    // for x in 0..data[0].len()-2 {
    //     for y in 0..data.len()-2 {
    //         if (data[y+2][x] == 'M' && data[y+1][x+1] == 'A' && data[y][x+2] == 'S' ){
    //             total += 1;
    //         }
    //         if (data[y+2][x] == 'S' && data[y+1][x+1] == 'A' && data[y][x+2] == 'M' ){
    //             total += 1;
    //         }
    //     }
    // }

    println!("{}", total);
    
    


}