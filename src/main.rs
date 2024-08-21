use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    // println!("correct value: {correct}\n");
    println!("Hey, guess a number between 1 - 10:\n");
    
    
    loop {
        let mut num_guessed = String::new();

        io::stdin()
            .read_line(&mut num_guessed)
            .expect("Error reading input");

        
        let num_guessed: u32 = match num_guessed.trim().parse() {
            Ok(parsed_num) => parsed_num,
            Err(e) => {
                println!("Error with parse, try again. {e}");
                continue;
            }
        };


        match num_guessed.cmp(&correct) {
            Ordering::Greater => println!("You guessed too high!"),
            Ordering::Less => println!("You guessed too low..."),
            Ordering::Equal => {
                println!("You guessed the correct number!");
                break;
            }
        };
    }
    
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