use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut amount_of_numbers_str = String::new();
    println!("How many random numbers do you want to guess?");

    io::stdin()
        .read_line(&mut amount_of_numbers_str)
        .expect("Error reading amount of numbers input");

    let amount_of_numbers: u8 = amount_of_numbers_str
        .trim()
        .parse()
        .expect("Error parsing str to a number");

    let mut correct_answers = Vec::new();

    for _ in 0..amount_of_numbers {
        correct_answers.push(rand::thread_rng().gen_range(1..=10));
    }

    println!("Hey, guess a number between 1 - 10:\n");

    let mut correctly_answered: u8 = 0;

    while correctly_answered < amount_of_numbers {
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

        match num_guessed.cmp(&correct_answers[correctly_answered as usize]) {
            Ordering::Greater => println!("You guessed too high!"),
            Ordering::Less => println!("You guessed too low..."),
            Ordering::Equal => {
                println!("You guessed the correct number!");
                correctly_answered += 1;
            }
        };
    }
    println!("Well done! You guessed all of these numbers:");
    for answer in correct_answers {
        println!("{answer}")
    }
    let mut basic_nums = Vec::new();
    basic_nums = [1, 2, 3, 4, 5].to_vec();
    for num in basic_nums {
        println!("{num}")
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
