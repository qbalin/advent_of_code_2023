use regex::Regex;
use super::file_reader;

const RED_CUBES_COUNT: u32 = 12;
const GREEN_CUBES_COUNT: u32 = 13;
const BLUE_CUBES_COUNT: u32 = 14;

pub fn run() -> std::io::Result<u32> {
    let mut possible_games: Vec<u32> = vec![];
    let _ = file_reader::lines("inputs/day2_input.txt", |line| {
        let game_and_data: Vec<&str> = line.split(':').collect();
        let game_metadata = game_and_data[0];
        let data = game_and_data[1];
        let sub_games: Vec<&str> = data.split(';').collect();
        let mut all_subgames_are_possible = true;

        for sub_game_data in sub_games {
            let red_re = Regex::new(r"([0-9]+) red").unwrap();
            for (_, [reds]) in red_re.captures_iter(sub_game_data).map(|caps| caps.extract()) {
                let integer = reds.parse::<u32>().unwrap();
                if integer > RED_CUBES_COUNT {
                    all_subgames_are_possible = false;
                    break;
                }
            }
            if !all_subgames_are_possible {
                break;
            }

            let green_re = Regex::new(r"([0-9]+) green").unwrap();
            for (_, [greens]) in green_re.captures_iter(sub_game_data).map(|caps| caps.extract()) {
                let integer = greens.parse::<u32>().unwrap();
                if integer > GREEN_CUBES_COUNT {
                    all_subgames_are_possible = false;
                    break;
                }
            }
            if !all_subgames_are_possible {
                break;
            }

            let blue_re = Regex::new(r"([0-9]+) blue").unwrap();
            for (_, [blues]) in blue_re.captures_iter(sub_game_data).map(|caps| caps.extract()) {
                let integer = blues.parse::<u32>().unwrap();
                if integer > BLUE_CUBES_COUNT {
                    all_subgames_are_possible = false;
                    break;
                }
            }
            if !all_subgames_are_possible {
                break;
            }

        }
        if all_subgames_are_possible {
            let game_number = (game_metadata.split(' ').collect::<Vec<&str>>())[1].parse::<u32>().unwrap();
            possible_games.push(game_number);
        }
    });

    let result = possible_games.iter().sum::<u32>();

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_outputs_the_proper_result() {
        assert_eq!(run().unwrap(), 2632);
    }
}

