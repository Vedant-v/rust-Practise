use std::{io,cmp::Ordering};
use rand;
fn main() {
    println!("wellcome to gessing game");
    let secrite =rand::random_range(1..=100);
    println!("the secrite value is {secrite}");
    loop{
        println!("Please enter the number:");
        let mut gesses = String::new();
    
        io::stdin()
            .read_line(&mut gesses)
            .expect("cant read your input");
        let gesses: u32 =gesses.trim().parse().{
            Ok(num) => num,
            Err(_) => continue;
        };
        println!("you gessed : {gesses}");
    
        match gesses.cmp(&secrite){
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
