pub trait Output {
    fn write(&self, message: &str);
}

pub struct ConsoleOutput;

impl Output for ConsoleOutput {
    fn write(&self, message: &str) {
        println!("{}", message);
    }
}

pub struct MockOutput {
    pub expected_output: &'static str,
}

impl Output for MockOutput {
    fn write(&self, string: &str) {
        assert_eq!(self.expected_output, string);
    }
}

impl Default for MockOutput {
    fn default() -> Self {
        MockOutput {
            expected_output: "",
        }
    }
}
