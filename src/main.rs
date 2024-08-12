use ferris_says::say;
use std::io::{stdout, BufWriter};
fn main() {
    let stdout = stdout();

    // Concatting strings
    let first_name = "Aidan";
    let last_name = "Lowson";
    let message = ["Hello, from", first_name, last_name].join(" ");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    // Looping through an arr
    let nums = [1, 2, 3, 4, 5];
    println!("{nums:?}");
}