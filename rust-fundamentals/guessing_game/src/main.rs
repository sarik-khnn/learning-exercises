use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
fn main() {
    println!("-----guessing game-----");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_number = String::new();
    loop {
        print!("please enter your guess in range of 1 to 100: ");
        io::stdout().flush().unwrap();
        guess_number.clear(); 
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Error:can't read input---program crashes---");
        match guess_number.trim().parse::<u32>() {
            Ok(number) => {
                if number < 1 || number > 100 {
                    println!("please enter your guess in range of 1 to 100!");
                    continue;
                } 
                match number.cmp(&secret_number){
                    Ordering::Less => println!("guess is small please enter a big value!"),
                    Ordering::Greater => println!("guess is big please enter a small value!"),
                    Ordering::Equal => {
                        println!("congratulation you won the match the secret number is: {}!",number);
                        break;
                    }
                }
            },
            Err(_) => println!("please enter a positive number!"),
        }
    }
}
