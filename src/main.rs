
use std::{io::{BufReader, BufRead}, fs::File, collections::{VecDeque, HashMap}};

const BACKSPACE: char = 8u8 as char;

const ENCRYPTION_KEY: i64 = 811589153;


fn mix(encrypted_file: &Vec<i64>, working_file: &mut VecDeque<(usize, i64)>){

    for (i, element) in encrypted_file.iter().enumerate() {

        let old_index = working_file.iter().position(|v| v == &(i, *element)).unwrap();

        if element == &0 {
            continue;
        } else  {
            let new_index = (old_index as i64 + element - 1).rem_euclid(encrypted_file.len() as i64 - 1) + 1;
            working_file.remove(old_index);
            working_file.insert(new_index as usize, (i, *element));
        }
    }
}


fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let encrypted_file : Vec<i64> = reader.lines().map(
      |l| l.unwrap().parse::<i64>().unwrap() * ENCRYPTION_KEY).collect();

    let mut working_file: VecDeque<(usize, i64)> = encrypted_file.iter().enumerate().map(|(i, v)| (i, *v)).collect();

    for _ in 0..10 {
        mix(&encrypted_file, &mut working_file);
    }
    
    let mixed_file: Vec<i64> = working_file.iter().map(|(_, v)| *v).collect();

    let zero_index = mixed_file.iter().position(|v| v == &0).unwrap();
    let first_shift = mixed_file[(zero_index + 1000) % mixed_file.len()];
    let second_shift = mixed_file[(zero_index + 2000) % mixed_file.len()];
    let third_shift = mixed_file[(zero_index + 3000) % mixed_file.len()];
    
    println!("final result {}",  first_shift + second_shift + third_shift);
}