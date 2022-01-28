use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let sercret_num = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Please input guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed:{} ", guess);
        match guess.cmp(&sercret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
            }
        }
    }
}
