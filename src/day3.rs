use super::file_reader::lines;
const NUMBERS_AS_STRING: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn run() -> std::io::Result<u64> {
    let mut matrix: Vec<Vec<char>> = vec![];
    let file = lines("inputs/day3_input.txt", |line| {
        let mut all_chars: Vec<char> = vec![];
        for c in line.chars() {
            all_chars.push(c);
        }
        matrix.push(all_chars);
    });

    let mut res: u64 = 0;
    for (row_index, row) in matrix.iter().enumerate() {
        let mut num = "".to_string();
        for (col_index, c) in row.iter().enumerate() {
            if NUMBERS_AS_STRING.contains(c) {
                num = format!("{num}{c}");
            }
            if num.len() > 0 && (col_index == (row.len() - 1) || !NUMBERS_AS_STRING.contains(&row[col_index + 1])) {
                for i in row_index.saturating_sub(1)..=(row_index + 1).min(matrix.len() - 1) {
                    for j in col_index.saturating_sub(num.len())..=(col_index + 1).min(row.len() - 1) {
                        if matrix[i][j] != '.' && !NUMBERS_AS_STRING.contains(&matrix[i][j]) {
                            res += num.parse::<u64>().unwrap_or(0);
                        }
                    }
                }
                num = "".to_string();
            }
        }
    }

    Ok(res)
}

pub fn run2() -> std::io::Result<u64> {
    let mut matrix: Vec<Vec<char>> = vec![];
    let file = lines("inputs/day3_input.txt", |line| {
        let mut all_chars: Vec<char> = vec![];
        for c in line.chars() {
            all_chars.push(c);
        }
        matrix.push(all_chars);
    });

    let mut res: u64 = 0;
    for (row_index, row) in matrix.iter().enumerate() {
        let mut num = "".to_string();
        for (col_index, c) in row.iter().enumerate() {
            if NUMBERS_AS_STRING.contains(c) {
                num = format!("{num}{c}");
            }
            if num.len() > 0 && (col_index == (row.len() - 1) || !NUMBERS_AS_STRING.contains(&row[col_index + 1])) {
                for i in row_index.saturating_sub(1)..=(row_index + 1).min(matrix.len() - 1) {
                    for j in col_index.saturating_sub(num.len())..=(col_index + 1).min(row.len() - 1) {
                        if matrix[i][j] != '.' && !NUMBERS_AS_STRING.contains(&matrix[i][j]) {
                            res += num.parse::<u64>().unwrap_or(0);
                        }
                    }
                }
                num = "".to_string();
            }
        }
    }

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_outputs_the_proper_result() {
        assert_eq!(run().unwrap(), 553079);
    }
}

