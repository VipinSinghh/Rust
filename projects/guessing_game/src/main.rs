use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, rust!");
    

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");
    
    loop {

        println!("Please, input your guess!");

        let mut guess = String::new();
        io::stdin()
                .read_line(&mut guess)
                .expect("failed to read value");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("you guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too less"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
