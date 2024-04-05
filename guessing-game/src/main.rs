use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to guessing game!");

    let secret_num = rand::thread_rng().gen_range(0..=100);

    println!("The secret number is {secret_num}");

    loop {
        println!("Please enter your number:");

        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error while reading line");

        let num: u32 = match num.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                if num.trim().eq("quit") {
                    println!("Ok, exiting");
                    break;
                }
                continue;
            }
        };

        println!("You guessed {num}");

        match num.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations!!!");
                break;
            },
        }
    }
}
