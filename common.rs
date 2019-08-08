use std::fs::File;
use std::io::{stdout, BufWriter, Read, Write};
use std::thread;

fn main() {
    thread::Builder::new()
        .stack_size(32 * 1024 * 1024) // 32MB
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}

/// ローカル環境なら _in.txt から入力を受ける
fn read_input() -> String {
    let stdin = std::io::stdin();
    let mut s = String::new();
    match option_env!("LOCAL") {
        Some(_) => File::open("src/_in.txt")
            .expect("File not found")
            .read_to_string(&mut s)
            .unwrap(),
        None => stdin.lock().read_to_string(&mut s).unwrap(),
    };
    s
}

fn run() {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let input = read_input();
    let mut input = input.split_whitespace();
}
