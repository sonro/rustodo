use std::io;

pub fn get_cli_input(prompt: &str) -> String {
    get_input(prompt, io::stdin().lock(), io::stdout())
}

fn get_input(prompt: &str, mut reader: impl io::BufRead, mut writer: impl io::Write) -> String {
    write!(writer, "{}", prompt).ok();
    writer.flush().ok();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.pop();
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let test_input = String::from("Test input\n");
        let expected_input = &test_input[..test_input.len() - 1];
        let reader = io::Cursor::new(test_input.as_bytes());

        let prompt = "Test prompt: ";
        let mut writer = Vec::new();

        let input = get_input(prompt, reader, &mut writer);
        assert_eq!(prompt.as_bytes(), writer);
        assert_eq!(expected_input, input);
    }
}
