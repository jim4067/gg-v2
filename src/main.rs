//Guessing game -> James Mutuku. reg no. I44/2397/2019

use rand::Rng;
use std::{cmp::Ordering, io, process};

//to run or not to run
fn run() {
    println!("Do you want to play the guessing game? (Y / N)");

    let prompt_play = parse_input();

    if prompt_play.as_str().to_lowercase().contains("y") {
        guess();
    } else {
        println!("I see you chickened out, see you later");
        process::exit(1);
    }
}

fn main() {
    run();
}

//main function guessing the number
fn guess() {
    println!("Great! guess the number ");
    loop {
        let secret_number = rand::thread_rng().gen_range(1..8);

        let user_input = parse_input();

        let user_guess = user_input.trim().parse::<i64>().unwrap_or_else(|err| {
            eprintln!("error while processing user's guess -> {}", err);
            process::exit(1);
        });

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("aim higher"),
            Ordering::Greater => println!("Okay go a little lower"),
            Ordering::Equal => {
                println!("Cooooorrect, you won!");
                break;
            }
        }
    }
}

//fn to get user input
fn parse_input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("error while reading from stdin");

    user_input
}
