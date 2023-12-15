use tetris::{file_system::FileSystemOperations, main_handler};

struct MockFileSystem {
    is_exist: bool,
    file_content_result: Result<String, std::io::Error>,
    expected_output: &'static str,
    write_file_result: Result<(), std::io::Error>,
}

impl FileSystemOperations for MockFileSystem {
    fn exists(&self, _file_path: &str) -> bool {
        self.is_exist
    }
    fn read_file(&self, _file_path: &str) -> Result<String, std::io::Error> {
        match self.file_content_result {
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Couldn't read file",
            )),
            Ok(ref value) => Ok(value.clone()),
        }
    }
    fn write_file(&mut self, _file_path: &str, content: &str) -> Result<(), std::io::Error> {
        assert_eq!(content, self.expected_output);

        match self.write_file_result {
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Couldn't save file",
            )),
            Ok(_) => Ok(()),
        }
    }
}

impl Default for MockFileSystem {
    fn default() -> Self {
        MockFileSystem {
            is_exist: false,
            file_content_result: Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Couldn't read file",
            )),
            expected_output: "",
            write_file_result: Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use tetris::output::MockOutput;

    use super::*;

    #[test]
    fn should_out_usage_info_if_filepath_absent() {
        let args: Vec<String> = vec![];

        main_handler(
            args,
            &mut MockFileSystem::default(),
            &mut MockOutput {
                expected_output: "Usage: ./main <filename>",
            },
        );
    }

    #[test]
    fn should_output_error_if_file_not_exists() {
        let args: Vec<String> = vec!["messi.txt".to_string()];

        let mut mock_file_system = MockFileSystem {
            is_exist: false,
            ..Default::default()
        };

        let mut mock_output = MockOutput {
            expected_output: "File not exists",
        };

        main_handler(args, &mut mock_file_system, &mut mock_output)
    }

    #[test]
    fn should_output_error_if_could_not_read_file() {
        let args: Vec<String> = vec!["messi.txt".to_string()];

        let mut mock_file_system = MockFileSystem {
            file_content_result: Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Couldn't read file",
            )),
            is_exist: true,
            ..Default::default()
        };

        let mut mock_output = MockOutput {
            expected_output: "Couldn't read file",
        };

        main_handler(args, &mut mock_file_system, &mut mock_output)
    }

    #[test]
    fn should_correctly_play_game() {
        let input = r"3 4
        .p.
        pp.
        ...
        ###"
        .to_string();

        let output_str = "...
.p.
pp.
###
";

        let args: Vec<String> = vec!["messi.txt".to_string()];

        let mut mock_file_system = MockFileSystem {
            file_content_result: Ok(input),
            is_exist: true,
            expected_output: output_str,
            ..Default::default()
        };

        main_handler(
            args,
            &mut mock_file_system,
            &mut MockOutput {
                expected_output: "File created",
            },
        );
    }

    #[test]
    fn should_display_error_message_on_file_not_saved() {
        let output_str = "...
.p.
pp.
###
";
        let input = r"3 4
        .p.
        pp.
        ...
        ###"
        .to_string();

        let args = vec!["messi.txt".to_string()];
        let mut mock_file_system = MockFileSystem {
            file_content_result: Ok(input),
            is_exist: true,
            expected_output: output_str,
            write_file_result: Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Couldn't save file",
            )),
            ..Default::default()
        };

        let mut mock_output = MockOutput {
            expected_output: "Couldn't save file",
        };

        main_handler(args, &mut mock_file_system, &mut mock_output)
    }
}
