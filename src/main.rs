use std::io;
use std::process::Command;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Failed to read line");

    let command = input.trim();

    Command::new(command)
        .spawn()
        .expect("Error: Failed to spawn process");
}
