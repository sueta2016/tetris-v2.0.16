use std::env;

use tetris::{file_system, main_handler, output, Config};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config {
        show_steps: args.contains(&"--steps".to_string()),
        file_path: if args.is_empty() {
            String::new()
        } else {
            args[0].clone()
        },
    };

    main_handler(
        config,
        &mut file_system::FileSystem,
        &mut output::ConsoleOutput,
    );
}
