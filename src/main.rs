pub mod repl;
pub mod command;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
