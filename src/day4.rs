use super::file_reader;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>
}

impl Card {
    pub fn count_points(&self) -> u32 {
        let set_of_winning_numbers: HashSet<u32> = self.winning_numbers.iter().copied().collect();
        let set_of_numbers: HashSet<u32> = self.numbers.iter().copied().collect();
        let matches = set_of_winning_numbers.intersection(&set_of_numbers).collect::<Vec<_>>().len() as u32;
        if matches == 0 {
            return 0;
        } else {
            return 2u32.pow(matches - 1);
        }

    }
}

fn parse_input(input: &str) -> Result<Card, std::num::ParseIntError> {
    let game_and_data: Vec<&str> = input.split(':').collect();
    let data = game_and_data[1];
    let number_sets: Vec<&str> = data.split('|').collect();
    let winning_numbers: Vec<u32> = number_sets[0].split(' ').filter(|a| !a.is_empty() ).map(|num| num.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let numbers = number_sets[1].split(' ').filter(|a| !a.is_empty() ).map(|num| num.trim().parse::<u32>().unwrap()).collect::<Vec<u32>>();

    Ok(Card {
        winning_numbers,
        numbers
    })
}

pub fn run() -> Result<u32, std::io::Result<u32>> {
    let mut points = 0;
    let _ = file_reader::lines("inputs/day4_input.txt", |line| {
        points += parse_input(line).unwrap().count_points();
        println!("card: {:?}, points: {}\n", parse_input(line), parse_input(line).unwrap().count_points())
    });

    Ok(points)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_outputs_the_proper_result() {
        assert_eq!(run().unwrap(), 21558);
    }
}

