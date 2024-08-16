use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    let say_extracted = say(&message, width, &mut writer);
    say_extracted.unwrap();
}
/*
https://www.rust-lang.org/learn/get-started
*/