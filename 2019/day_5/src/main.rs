use std::fs;
use crate::computer::{Computer};

mod computer;



fn main() {
    let input_data = fs::read_to_string("input.txt").expect("File doesn't exist");

    let initial_memory: Vec<i32> = input_data.trim().split(',').map(|s| s.parse().unwrap()).collect();

    let mut computer: Computer = Computer::new(initial_memory.clone(), vec![1]);
    computer.execute();
    println!("{:?}", computer.output);

    let mut computer2: Computer = Computer::new(initial_memory.clone(), vec![5]);
    computer2.execute();
    println!("{:?}", computer2.output);
}
