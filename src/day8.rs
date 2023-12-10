use std::fs;
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str
}

pub fn run() -> std::io::Result<i64> {
    let content = fs::read_to_string("inputs/day8_input.txt")?;
    let mut hash_map: HashMap<String, Node> = HashMap::new();
    let mut lines = content.split('\n');

    let lr = lines.next().unwrap().chars();
    for line in lines.skip(1) {
        let re = Regex::new(r"( = \(|, |\))").unwrap();
        let matches: Vec<&str> = re.split(line).collect();

        hash_map.insert(matches[0].to_string(), Node { left: matches[1], right: matches[2] });
    }

    let number_of_steps_to_go_from_aaa_to_zzz = number_of_steps_to_go_from_aaa_to_zzz(&hash_map, &lr);

    Ok(number_of_steps_to_go_from_aaa_to_zzz)
}

fn number_of_steps_to_go_from_aaa_to_zzz(hash_map: &HashMap<String, Node<'_>>, lr: &std::str::Chars<'_>) -> i64 {
    let mut current_node = hash_map.get("AAA").unwrap();
    let mut steps = 0;

    for left_or_right in lr.clone().cycle() {
        let new_value: &str = if left_or_right == 'L' {
            current_node.left
        } else {
            current_node.right
        };

        steps += 1;

        if new_value == "ZZZ" {
            break;
        }
        current_node = hash_map.get(new_value).unwrap();
    }


    steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_outputs_the_proper_result() {
        assert_eq!(run().unwrap(), 21883);
    }
}

