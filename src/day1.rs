use super::file_reader;

const NUMBERS_AS_STRING: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn run() -> std::io::Result<u32> {
    let mut numbers: Vec<u32> = vec![];
    let _ = file_reader::lines("inputs/day1_input.txt", |line| {
        let first_number = line.chars().find(|c| { NUMBERS_AS_STRING.contains(c) }).expect("Did not find a number in input");
        let last_number = line.chars().rev().find(|c| { NUMBERS_AS_STRING.contains(c) }).unwrap();

        let integer = format!("{first_number}{last_number}").parse::<u32>().unwrap();
        numbers.push(integer);
    });

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

