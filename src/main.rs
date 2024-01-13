use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);
    
    println!("Guess the number!");
    
    // println!("Secret number is: {secret_num}");
    
    loop {
        println!("Input your guess");
        let mut guess = String::new();
        
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
