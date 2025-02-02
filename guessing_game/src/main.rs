use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::{self, Colorize};


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess= String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,          // for value other than Integer
        };
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            },
        }
    

    }



}
