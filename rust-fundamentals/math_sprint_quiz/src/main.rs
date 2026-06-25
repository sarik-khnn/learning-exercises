use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
fn main() {
    let mut won: u32 = 0;
    let mut input_answer = String::new();

    println!("==========Rust Math Sprint Game==========");
    for game_round in 1..=4 {
        println!("----------Round {}----------", game_round);
        let mut operand1: f32 = rand::thread_rng().gen_range(1.0..=999.0);
        let mut operand2: f32 = rand::thread_rng().gen_range(1.0..=999.0);
        operand1 = (operand1 * 100.0).round() / 100.0;
        operand2 = (operand2 * 100.0).round() / 100.0;
        let (symbol, mut org_ans) = match game_round {
            1 => ("+", operand1 + operand2),
            2 => ("-", operand1 - operand2),
            3 => ("*", operand1 * operand2),
            4 => ("/", operand1 / operand2),
            _ => unreachable!(),
        };
        org_ans = (org_ans * 100.0).round() / 100.0;
        let mut chance = 3;

        loop {
            if chance == 0 {
                println!("You loose round {} The answer is: {}", game_round, org_ans);
                break;
            }
            println!("{:.2} {} {:.2} = ?", operand1, symbol, operand2);
            println!("Remaining chance: {}", chance);
            print!("please enter your answer: ");
            io::stdout().flush().unwrap();
            input_answer.clear();
            io::stdin()
                .read_line(&mut input_answer)
                .expect("Error!-> can't read input ! program crashing... !");
            match input_answer.trim().parse::<f32>() {
                Ok(answer) => match answer.partial_cmp(&org_ans).unwrap() {
                    Ordering::Less => {
                        println!("Your answer is small please enter a big answer!");
                        chance -= 1;
                        continue;
                    }
                    Ordering::Greater => {
                        println!("Your answer is big please enter a small answer!");
                        chance -= 1;
                        continue;
                    }
                    Ordering::Equal => {
                        println!("Your answer is correct the answer is: {}", answer);
                        won += 1;
                        break;
                    }
                },
                Err(_) => {
                    println!("please enter answer!");
                    chance += 1;
                    continue;
                }
            }
        }
    }

    println!("=========End=========");
    println!("your Score is: {}/4", won);
}