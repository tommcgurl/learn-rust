use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a number between 1 and 100
    let secret_number = rand::thread_rng()
        .gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // Create a mutable string.
        let mut guess = String::new();

        // Read user input from standard in and store in
        // the guess variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess which is a string into a u32
        // integer. We first have to trim it to remove the
        // breaking line character \n at the end.
        let guess: u32 = match guess.trim().parse() {
            // If we successfully parsed a number, store the number
            Ok(num) => num,
            // If a number wasn't sucessfully parsed, just continue.
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
              println!("You win!");
              break;
          }
        }
    }


    
}
