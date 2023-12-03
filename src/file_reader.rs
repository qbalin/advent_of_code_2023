use std::{fs::File, io::BufReader, io::BufRead};

pub fn lines(file_path: &str, mut closure: impl FnMut(&str) -> ()) -> std::io::Result<()> {
    let file = File::open(file_path).map_err(|err| panic!("File not found: {err}") ).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    while reader.read_line(&mut buffer)? > 0 {
        closure(buffer.trim());
        buffer.clear();
    }

    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_yields_each_line_of_the_file_to_the_closure() {
        let mut all_lines: Vec::<String> = vec![];
        let _ = lines("./src/file_reader.rs", |line| all_lines.push(line.to_owned()));
        assert_eq!(all_lines[0], "use std::{fs::File, io::BufReader, io::BufRead};".to_string());
        assert_eq!(all_lines[6], "while reader.read_line(&mut buffer)? > 0 {".to_string());
    }

    #[test]
    #[should_panic(expected = "File not found:")]
    fn returns_an_error_when_the_file_is_not_found() {
        let _ = lines("./src/unexisting_file.rs", |_x| () );
    }
}
