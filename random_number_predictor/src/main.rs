
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io,
};
use rand::Rng; 

fn main() {
    loop {

let n: u32 = rand::random_range(0..=100); 
        println!("Rnadom number {n}");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read i/p");
        //parsing to i32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("I am sorry {e}");
                continue;
            }
        };
        match guess.cmp(&n) {
            Equal =>{ println!("your won");break ;},
            Less => println!("less"),
            Greater => println!("more"),
        }
    }
}
