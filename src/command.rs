use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

pub fn read_file(file_name: &str) -> String {
    String::from_utf8(fs::read(file_name).unwrap_or_default()).unwrap()
}

pub fn write_result(file_name: &str, file_contents: &str) {
    let mut file = File::create(file_name).unwrap();
    file.write_all(file_contents.as_bytes()).unwrap();
}

pub fn add_command<'a>(file_lines: Vec<&'a str>, command: &'a String) -> Vec<&'a str> {
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

pub enum CommandExecuteResult {
    Success(String),
    Fail(String),
}

pub fn command_execute() -> CommandExecuteResult {
    let repl_path = "./repl-internals/repl-content.c";
    let prev_repl_path = "./repl-internals/prev-repl-content.c";
    let repl_exe = "./repl-internals/repl";

    let command_output;

    let compile_output = Command::new("gcc")
        .arg(repl_path)
        .arg("-orepl-internals/repl")
        .arg("-Werror=implicit-function-declaration")
        .output()
        .unwrap();

    if !compile_output.status.success() {
        command_output =
            CommandExecuteResult::Fail(String::from_utf8(compile_output.stderr.clone()).unwrap());

        fs::remove_file(repl_path).ok();
        fs::copy(prev_repl_path, repl_path).unwrap();
    } else {
        let result = Command::new(repl_exe).output();
        command_output =
            CommandExecuteResult::Success(String::from_utf8(result.unwrap().stdout).unwrap());

        fs::copy(repl_path, prev_repl_path).ok();
    }

    command_output
}

fn find_substring_index(haystack: &str, needle: &str) -> Option<usize> {
    haystack[needle.len()..]
        .find(needle)
        .map(|idx| idx + needle.len())
}

pub fn format_error(error_msg: String) -> HashSet<Vec<String>> {
    let msg = error_msg.clone();
    let splitted: Vec<&str> = msg.split("\n").collect();

    let mut error_idxs = vec![];

    for (idx, msg) in splitted.iter().enumerate() {
        if msg.contains("error: ") {
            error_idxs.push(idx);
        }
    }

    let mut formatted_msgs = HashSet::new();

    for idx in error_idxs {
        let mut msg_vector = vec![];

        let needle = "error: ";
        let sub_str_idx = find_substring_index(splitted.get(idx).unwrap(), needle);
        let stripped_msg = &splitted.get(idx).unwrap()[sub_str_idx.unwrap()..];

        msg_vector.push(stripped_msg);

        let sub_str_idx = find_substring_index(splitted.get(idx + 1).unwrap(), "| ");

        msg_vector.push(&splitted.get(idx + 1).unwrap()[sub_str_idx.unwrap() + "| ".len()..]);
        msg_vector.push(&splitted.get(idx + 2).unwrap()[sub_str_idx.unwrap() + "| ".len()..]);

        formatted_msgs.insert(msg_vector.iter().map(|s| s.to_string()).collect());
    }

    formatted_msgs
}
