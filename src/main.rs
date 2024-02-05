use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;

fn read_file(file_name: &str) -> String {
    String::from_utf8(fs::read(file_name).unwrap()).unwrap()
}

fn write_result(file_contents: String) {
    fs::remove_file("./repl-content.c").ok();
    fs::write("./repl-content.c", file_contents).unwrap();
}

fn add_command<'a>(file_lines: Vec<&'a str>, command: &'a String) -> Vec<&'a str> {
    let mut res: Vec<&str> = vec![];
    for line in file_lines {
        if line.contains("// write here") {
            res.push(&command);
            continue;
        }

        if line.contains("return 0") {
            res.push("// write here");
        }

        res.push(line);
    }
    res
}

// might be useful later
fn find_error_line(error_output: &String) -> usize {
    let lines: Vec<_> = error_output.split("\n").collect();

    let error_msg_prefix = "repl-content.c:";
    let err_line = lines.get(1).unwrap().to_string();
    let line_number_idx = err_line
        .find(error_msg_prefix)
        .map(|idx| idx + error_msg_prefix.len())
        .unwrap();

    let mut res_lit = String::new();

    for ch in err_line[line_number_idx..]
        .chars()
        .by_ref()
        .take_while(|c| c.is_numeric())
    {
        res_lit.push(ch);
    }

    res_lit.parse::<usize>().unwrap()
}

fn main() {
    let file_content = read_file("repl-content.c");
    let file_lines: Vec<&str> = file_content.split('\n').collect();

    let command = "printf(\"%d\", a);".to_string();
    let modified_file = add_command(file_lines, &command);

    write_result(modified_file.join("\n"));

    let compile_output = Command::new("gcc")
        .arg("repl-content.c")
        .arg("-orepl")
        .output()
        .unwrap();

    if !compile_output.status.success() {
        let error_msg = String::from_utf8(compile_output.stderr.clone()).unwrap();
        find_error_line(&error_msg);
        println!("{}", error_msg);

        fs::remove_file("./repl-content.c").ok();
        fs::copy("./previous-repl-content.c", "./repl-content.c").unwrap();
    } else {
        let result = Command::new("./repl").output();
        let output = String::from_utf8(result.unwrap().stdout).unwrap();
        println!("{}", output);

        // if everything was successful, we copy the working version to a temporary file
        fs::copy("./repl-content.c", "./previous-repl-content.c").ok();
    }

    // loop {
    //     let mut buffer = String::new();

    //     print!(">> ");
    //     io::stdout().flush().unwrap();
    //     io::stdin()
    //         .read_line(&mut buffer)
    //         .expect("Error reading from STDIN");

    // }
}
