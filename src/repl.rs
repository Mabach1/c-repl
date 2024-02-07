use crate::command::*;
use std::{
    fs,
    io::{self, Write},
};

pub struct REPL {
    // command_buffer: Vec<String>, // <- for command history
}

impl REPL {
    pub fn new() -> Self {
        Self {
            // command_buffer: vec![],
        }
    }

    fn repl_internal_reset() {
        let repl_path = "./repl-internals/repl-content.c";
        let prev_repl_path = "./repl-internals/prev-repl-content.c";

        let default_setup = r#"int main(void) {
    // write here
    return 0;
}"#;
        // not sure about how effective this approach is, might change later
        fs::remove_file(repl_path).ok();
        fs::remove_file(prev_repl_path).ok();

        fs::write(repl_path, default_setup).unwrap();
        fs::write(prev_repl_path, default_setup).unwrap();
    }

    pub fn run(&mut self) {
        REPL::repl_internal_reset();

        loop {
            let repl_path = "./repl-internals/repl-content.c";
            let mut buffer = String::new();

            print!("C > ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Error reading from STDIN");

            let command = buffer.trim_end().to_string();

            if command == ":q" {
                break;
            }

            let file_content = read_file(repl_path);
            let file_lines: Vec<&str> = file_content.split('\n').collect();

            let modified_file = add_command(file_lines, &command);

            write_result(repl_path, &modified_file.join("\n"));

            match command_execute() {
                CommandExecuteResult::Success(output) => {
                    if !output.is_empty() {
                        println!("{}", output);
                    }
                }
                CommandExecuteResult::Fail(err_msg) => {
                    let formatted_err = format_error(err_msg);

                    formatted_err.iter().for_each(|msgs| {
                        for msg in msgs {
                            println!("{}", msg);
                        }
                    })
                }
            }
        }

        REPL::repl_internal_reset();
    }
}
