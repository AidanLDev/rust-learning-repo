use std::io;
use rand::Rng;

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("correct value: {correct}\n");
    println!("Hey, guess a number:\n");
    
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading input");

    println!("You guessed: {}", num.trim())
}

/*
use ferris_says::say;
use std::io::{stdout, BufWriter};
fn concat_example() {
    let stdout = stdout();

    // Concatting strings
    let first_name = "Aidan";
    let last_name = "Lowson";
    let message = ["Hello, from", first_name, last_name].join(" ");
    let width = message.chars().count();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
*/

/*
fn loop_example() {
    // Looping through an arr
    let nums = [1, 2, 3, 4, 5];
    println!("{nums:?}");
}
*/
/*
fn taking_user_input_example() {
       // Handlding data we're not sure about the value of
       print!("Enter the a user name: \n");

       let mut user_name = String::new();
   
       io::stdin().read_line(&mut user_name).expect("Error reading input");
   
       println!("Hello {}, Welcome!", user_name.trim())
}
*/