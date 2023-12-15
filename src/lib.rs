use field::parse_into_field;
use file_system::FileSystemOperations;
use output::Output;

pub mod field;
pub mod file_system;
pub mod output;

#[derive(Debug)]
pub struct Config {
    pub show_steps: bool,
    pub file_path: String,
}

pub fn main_handler(
    config: Config,
    file_system: &mut dyn FileSystemOperations,
    output: &mut dyn Output,
) {
    if config.file_path.is_empty() {
        output.write("Usage: ./main <filename>");
        return;
    }
    let file_path = config.file_path.as_str();
    // read file

    if !file_system.exists(&file_path) {
        output.write("File not exists");
        return;
    }

    let input = match file_system.read_file(file_path) {
        Ok(value) => value,
        Err(_) => {
            output.write("Couldn't read file");
            return;
        }
    };

    let mut field = match parse_into_field(input.as_str()) {
        Ok(value) => value,
        Err(err_text) => {
            output.write(err_text);
            return;
        }
    };
    // play game

    let mut step = 0;
    let mut all_steps_state = format!("STEP {0}:\n{1}\n", step, field.to_string());

    while field.can_move() {
        field.move_figure();
        step += 1;
        all_steps_state += format!("STEP {0}:\n{1}\n", step, field.to_string()).as_str();
    }

    let output_str = if config.show_steps {
        all_steps_state
    } else {
        field.to_string()
    };

    output.write(&output_str);
}
