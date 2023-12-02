use std::{fs::File, io::BufReader, io::BufRead};

const NUMBERS_AS_STRING: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn run() -> std::io::Result<u32> {
    let file = File::open("inputs/day1_input.txt").map_err(|err| panic!("File not found: {err}") ).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut numbers: Vec<u32> = vec![];
    while reader.read_line(&mut buffer)? > 0 {
        let first_number = buffer.chars().find(|c| { NUMBERS_AS_STRING.contains(c) }).expect("Did not find a number in input");
        let last_number = buffer.chars().rev().find(|c| { NUMBERS_AS_STRING.contains(c) }).unwrap();
        buffer.clear();

        let integer = format!("{first_number}{last_number}").parse::<u32>().unwrap();
        numbers.push(integer);
    }

    let result = numbers.iter().sum::<u32>();

    println!("Result: {result}");

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_outputs_the_proper_result() {
        assert!(run().unwrap() == 55477);
    }
}

