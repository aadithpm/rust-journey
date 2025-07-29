use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
        Ordering::Less => println!("less"),
        Ordering::Equal => {
            println!("winner!");
            break;
        },
        Ordering::Greater => println!("greater"),
        };
    }
}