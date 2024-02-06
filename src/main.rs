use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command;

static REPL_PATH: &'static str = "./repl-internals/repl-content.c";
static PREV_REPL_PATH: &'static str = "./repl-internals/prev-repl-content.c";
static REPL_EXE: &'static str = "./repl-internals/repl";

fn read_file(file_name: &str) -> String {
    String::from_utf8(fs::read(file_name).unwrap_or_default()).unwrap()
}

fn write_result(file_name: &str, file_contents: &str) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(file_contents.as_bytes()).unwrap();
}

fn add_command<'a>(file_lines: Vec<&'a str>, command: &'a String) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];
    let mut preprocessor = false;

    if command.contains("#define") || command.contains("#include") {
        res.push(&command);
        preprocessor = true;
    }

    for line in file_lines {
        if line.contains("printf") {
            continue;
        }

        if line.contains("// write here") && !preprocessor {
            res.push(&command);
        }

        res.push(line);
    }
    res
}

fn command_execute() -> String {
    let command_output;

    let compile_output = Command::new("gcc")
        .arg(REPL_PATH)
        .arg("-orepl-internals/repl")
        .arg("-Werror=implicit-function-declaration")
        .output()
        .unwrap();

    if !compile_output.status.success() {
        command_output = String::from_utf8(compile_output.stderr.clone()).unwrap();

        fs::remove_file(REPL_PATH).ok();
        fs::copy(PREV_REPL_PATH, REPL_PATH).unwrap();
    } else {
        let result = Command::new(REPL_EXE).output();
        command_output = String::from_utf8(result.unwrap().stdout).unwrap();

        fs::copy(REPL_PATH, PREV_REPL_PATH).ok();
    }

    command_output
}

fn repl_internal_reset() {
    let default_setup = r#"int main(void) {
    // write here
    return 0;
}"#;
    // not sure about how effective this approach is, might change later
    fs::remove_file(REPL_PATH).ok();
    fs::remove_file(PREV_REPL_PATH).ok();

    fs::write(REPL_PATH, default_setup).unwrap();
    fs::write(PREV_REPL_PATH, default_setup).unwrap();
}

fn main() {
    repl_internal_reset();

    loop {
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

        let file_content = read_file(REPL_PATH);
        let file_lines: Vec<&str> = file_content.split('\n').collect();

        let modified_file = add_command(file_lines, &command);

        write_result(REPL_PATH, &modified_file.join("\n"));

        let command_result = command_execute();

        if !command_result.is_empty() {
            println!("{}", command_result);
        }
    }

    repl_internal_reset();
}
