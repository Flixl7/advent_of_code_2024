use core::panic;
use std::{cmp::Ordering, fs};

fn main() {
    let mut total_diff = 0;
    let mut vec1:Vec<u32> = Vec::new();
    let mut vec2:Vec<u32> = Vec::new();
    let input = read_input();
    
    for line in input.lines() {
        let mut line_iter = line.split_whitespace();
        let num1: u32 = match line_iter.next() {
            Some(num) => match num.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("error in int conversion num2"),
            }
            None => panic!("test"),
        };
        let num2: u32 = match line_iter.next() {
            Some(num) => match num.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("error in int conversion num2"),
            }
            None => panic!("test"),
        };
        vec1.push(num1);
        vec2.push(num2);
    }

    vec1.sort_unstable();
    vec2.sort_unstable();

    let mut i = 0;
    for _line in input.lines(){
        let num1 = vec1[i];
        let num2 = vec2[i];

        let diff = match num1.cmp(&num2) {
            Ordering::Greater => num1 - num2,
            Ordering::Less => num2 - num1,
            Ordering::Equal => 0,
        };
        total_diff += diff;
        println!("{num1}, {num2}, {diff}");
        i+=1;
    }
    println!("{total_diff}");
}

fn read_input() -> String{
    let filepath = "input";
    let contents = match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(_) => panic!("error in reading file to string")
    };
    contents
}
