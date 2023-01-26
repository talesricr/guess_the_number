use rand::Rng; // 0.8.5
use std::io;

fn main() {
    let mut input = String::new();
    let randnumber = rand::thread_rng().gen_range(0..100);
    let mut number = 0;
    let mut tries = 0;
    while number != randnumber {
        println!("Enter a guess between 1 and 100: ");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        number = input.trim().parse().expect("Input not an integer");
        
        if number > randnumber {
            println!("To high!");
        }
        if number < randnumber {
            println!("To low!");
        }
        tries += 1;
        if number == randnumber {
            println!("You got it right! The correct answer is: {}", number);
            println!("Number of guesses: {}", tries);
            std::process::exit(0);
        }   
    }
}