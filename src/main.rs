use std::io;
use std::process::Command;

fn read_eval_print() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error: Failed to read line");

    let command = input.trim();

    Command::new(command)
        .spawn()
        .expect("Error: Failed to spawn process");
}

fn main() {
    loop {
        read_eval_print();
    }
}
