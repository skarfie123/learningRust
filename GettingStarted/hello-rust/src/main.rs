use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello World!");

    // String vs &str https://stackoverflow.com/a/24159933
    let message = String::from("Hello fellow Rustaceans!");
    // NOTE: message.len() is in bytes not characters
    let width = message.chars().count();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
