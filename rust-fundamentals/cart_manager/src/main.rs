use rand::Rng;
use std::io::{self, Write};
fn main() {
    let discount = rand::thread_rng().gen_range(5..=20);
    let mut item_price = String::new();
    let mut total_item: u32 = 0;
    let mut total_amount: u32 = 0;
    loop {
        print!("please enter item's price: ");
        io::stdout().flush().unwrap();
        item_price.clear();
        io::stdin()
            .read_line(&mut item_price)
            .expect("Error: can't read input ! program ceashing.. !");
        match item_price.trim() {
            "checkout" => break,
            _ => (),
        }
        match item_price.trim().parse::<u32>() {
            Ok(price) => {
                if price == 0 || price > 10000 {
                    println!("enter a valid price!");
                    continue;
                }
                total_amount += price;
                total_item += 1;
            }
            Err(_) => println!("please enter a valid price or checkout!"),
        }
    }

    let final_amount = (total_amount * (100 - discount)) / 100;
    println!("----------BILL----------");
    println!("total item scanned: {}", total_item);
    println!("total amount = {}", total_amount);
    println!("final amount with {} % OFF = {}", discount, final_amount);
}
