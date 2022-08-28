use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main(){
    println!("Guess the number!");
    println!("<--------------------->");
    
    let random_number = rand::thread_rng().gen_range(0..100);

    loop {
        println!("Input guess number: ");
        let mut guess: String = String::new();
        // Now we read the input
        io::stdin()
            .read_line(&mut guess)
            .expect("Ok");
        // Since guess is mutable, we can redefine it to an integer
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("<--------------------->");
        println!("Entered Number: {}", guess);
    
        match guess.cmp(&random_number) {
            Ordering::Less => println!("{}", "Number is smaller than the secret number!".red()),
            Ordering::Greater => println!("{}", "Number is greater than the secret number!".purple()),
            Ordering::Equal => {
                println!("{}", "You're goddamn right!".green());
                break;
            },
        }
        println!("<--------------------->");
    }

}
