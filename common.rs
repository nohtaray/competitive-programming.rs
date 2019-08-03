/// ローカル環境なら _in.txt から入力を受ける
fn read_input() -> String {
    use std::fs::File;
    use std::io::Read;
    let mut s = String::new();
    match option_env!("LOCAL") {
        Some(_) => File::open("src/_in.txt")
            .expect("File not found")
            .read_to_string(&mut s)
            .unwrap(),
        None => std::io::stdin().read_to_string(&mut s).unwrap(),
    };
    s
}

fn main() {
    //
}
