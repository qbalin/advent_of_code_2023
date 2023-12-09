use super::file_reader;
use std::iter::zip;

fn next_line(line: &Vec<i64>) -> Option<Vec<i64>> {
    let mut new_line = vec![];
    let mut all_zeroes = true;

    for (left, right) in zip(line, line.iter().skip(1)) {
        let diff = right - left;
        new_line.push(diff);
        if diff != 0 {
            all_zeroes = false;
        }
    }

    if all_zeroes {
        None
    } else {
        Some(new_line)
    }
}

pub fn run() -> std::io::Result<(i64, i64)> {
    let mut end_of_history: i64 = 0;
    let mut start_of_history: i64 = 0;
    let _ = file_reader::lines("inputs/day9_input.txt", |line| {
        let mut line = line.split(' ').map(|s| s.parse::<i64>().unwrap() ).collect::<Vec<i64>>();
        let mut first_element: Vec<i64> = vec![];
        let mut last_element: Vec<i64> = vec![];
        first_element.push(*line.first().unwrap());
        last_element.push(*line.last().unwrap());
        while let Some(next_line) = next_line(&line) {
            line = next_line;
            first_element.push(*line.first().unwrap());
            last_element.push(*line.last().unwrap());
        }
        end_of_history += last_element.iter().sum::<i64>();
        start_of_history += first_element.iter().rev().fold(0, |acc, i| i - acc);
    });

    Ok((start_of_history, end_of_history))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_outputs_the_proper_result() {
        assert_eq!(run().unwrap(), (928, 1974232246));
    }
}

