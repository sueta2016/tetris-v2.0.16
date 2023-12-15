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
    fn should_correctly_display_only_final_state() {
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

        mock_output
            .expect_write()
            .times(1)
            .with(eq(output_str))
            .returning(|_| ());

        main_handler(config, &mut mock_file_system, &mut mock_output);
    }

    #[test]
    fn should_correctly_dispalay_all_steps() {
        let input = r"3 7
        .p.
        pp.
        ...
        ...
        ...
        ...
        ###"
        .to_string();

        let output_str = r"STEP 0:
.p.
pp.
...
...
...
...
###

STEP 1:
...
.p.
pp.
...
...
...
###

STEP 2:
...
...
.p.
pp.
...
...
###

STEP 3:
...
...
...
.p.
pp.
...
###

STEP 4:
...
...
...
...
.p.
pp.
###

";

        let config = Config {
            show_steps: true,
            file_path: "messi.txt".to_string(),
        };

        let mut mock_file_system = MockFileSystemOperations::new();
        let mut mock_output = MockOutput::new();

        mock_file_system.expect_exists().times(1).return_const(true);
        mock_file_system
            .expect_read_file()
            .times(1)
            .returning(move |_| Ok(input.clone()));

        mock_output
            .expect_write()
            .times(1)
            .with(eq(output_str))
            .returning(|_| ());

        main_handler(config, &mut mock_file_system, &mut mock_output);
    }
}
