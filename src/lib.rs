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

    while field.can_move() {
        field.move_figure();
    }
    // write in file
    let final_state = field.to_string();

    match file_system.write_file("out.txt", final_state.as_str()) {
        Ok(_) => output.write("File created"),
        Err(_) => output.write("Couldn't save file"),
    }
}
