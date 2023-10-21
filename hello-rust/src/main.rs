use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();

    let mut buffer = String::from("hello new rustaceans");

    let count = buffer.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(&buffer, count, &mut writer).unwrap();
}