use std::io;
use rand::prelude::*;

fn main() {
    // standard input example
    //let mut buffer = String::new();
    //println!("Enter a message: ");
    //io::stdin().read_line(&mut buffer);
    //println!("{buffer}");

    //let number : i32 = buffer.trim().parse().unwrap();
    //println!("{}", number + 1);

    //random number example
    //let number = rand::random::<f64>();
    //println!("{number}");

    //let number = rand::thread_rng().gen_range(1..11);
    //println!("{number}");

    //Challenge
    let guessed_number : i16 = rand::thread_rng().gen_range(1..101);
    let mut finish = false;
    let mut guess : i16;
    let mut buffer = String::new();
    println!("Guess the number between 1 and 100");

    while !finish {
        buffer.clear();

        println!("Enter a number: ");
        io::stdin().read_line(&mut buffer);
        guess = buffer.trim().parse().unwrap();

        if guess < guessed_number
        {
            println!("Guess higher");
            continue;
        }
        if guess > guessed_number
        {
            println!("Guess lower");
            continue;
        }

        finish = true;
    }

    println!("Congratulations!")

}
