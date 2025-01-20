pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // The input is a Vec of tuples where each tuple contains a String and a Command
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // Output will be a vector of processed strings
        let mut output: Vec<String> = vec![];
        
        for (string, command) in input.iter() {
            let result = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(times) => format!("{}{}", string, "bar".repeat(*times)),
            };
            output.push(result);
        }
        
        output
    }
}

#[cfg(test)]
mod tests {
    // Import the transformer function
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
