use tetris::main_handler;

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use tetris::{file_system::MockFileSystemOperations, output::MockOutput, Config};

    use super::*;

    #[test]
    fn should_out_usage_info_if_filepath_absent() {
        let config: Config = Config {
            show_steps: false,
            file_path: "".to_string(),
        };

        let mut mock_file_system = MockFileSystemOperations::new();
        let mut mock_output = MockOutput::new();

        mock_output
            .expect_write()
            .times(1)
            .with(eq("Usage: ./main <filename>"))
            .returning(|_| ());

        main_handler(config, &mut mock_file_system, &mut mock_output);
    }

    #[test]
    fn should_output_error_if_file_not_exists() {
        let config: Config = Config {
            show_steps: false,
            file_path: "messi.txt".to_string(),
        };

        let mut mock_file_system = MockFileSystemOperations::new();
        let mut mock_output = MockOutput::new();

        mock_file_system
            .expect_exists()
            .times(1)
            .return_const(false);

        mock_output
            .expect_write()
            .times(1)
            .with(eq("File not exists"))
            .returning(|_| ());

        main_handler(config, &mut mock_file_system, &mut mock_output)
    }

    #[test]
    fn should_output_error_if_could_not_read_file() {
        let config: Config = Config {
            show_steps: false,
            file_path: "messi.txt".to_string(),
        };

        let mut mock_file_system = MockFileSystemOperations::new();
        let mut mock_output = MockOutput::new();

        mock_file_system.expect_exists().times(1).return_const(true);

        mock_file_system.expect_read_file().times(1).returning(|_| {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Couldn't read file",
            ))
        });

        mock_output
            .expect_write()
            .with(eq("Couldn't read file"))
            .returning(|_| ());

        main_handler(config, &mut mock_file_system, &mut mock_output)
    }

    #[test]
    fn should_correctly_play_game() {
        let input = r"3 4
            .p.
            pp.
            ...
            ###"
        .to_string();

        let output_str = r"...
.p.
pp.
###
";

        let config = Config {
            show_steps: false,
            file_path: "messi.txt".to_string(),
        };

        let mut mock_file_system = MockFileSystemOperations::new();
        let mut mock_output = MockOutput::new();

        mock_file_system.expect_exists().times(1).return_const(true);
        mock_file_system
            .expect_read_file()
            .times(1)
            .returning(move |_| Ok(input.clone()));

        mock_file_system
            .expect_write_file()
            .times(1)
            .with(eq("out.txt"), eq(output_str))
            .returning(|_, _| Ok(()));

        mock_output
            .expect_write()
            .times(1)
            .with(eq("File created"))
            .returning(|_| ());

        main_handler(config, &mut mock_file_system, &mut mock_output);
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

        let config = Config {
            show_steps: false,
            file_path: "messi.txt".to_string(),
        };

        let mut mock_file_system = MockFileSystemOperations::new();

        mock_file_system.expect_exists().times(1).return_const(true);
        mock_file_system
            .expect_read_file()
            .times(1)
            .returning(move |_| Ok(input.clone()));

        mock_file_system
            .expect_write_file()
            .times(1)
            .with(eq("out.txt"), eq(output_str))
            .returning(|_, _| {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Couldn't save file",
                ))
            });

        let mut mock_output = MockOutput::new();

        mock_output
            .expect_write()
            .times(1)
            .with(eq("Couldn't save file"))
            .returning(|_| ());

        main_handler(config, &mut mock_file_system, &mut mock_output)
    }
}
