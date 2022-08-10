use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    println!("Gessing Game!");

    let mut tries_left: i8 = 10;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please guess a number betwen 1 & 100: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

            
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("I said a number!  Hello?");
                continue;
            }, 
        };
        
        println!("You entered \"{}\"", guess);
        tries_left -= 1;
        let tries: i8 = 10 - tries_left;
        match guess.cmp(&secret_number) {
            Ordering::Less      => too_small(),
            Ordering::Greater   => too_big(),
            Ordering::Equal     => {
                win(tries);
                break;
            }
        }
        if tries_left < 0 {
            println!("You lose!");
            break;
        }
        println!("You have {} tries left.", tries_left);
    }

    fn too_small() {
        println!("Too small!");
    }

    fn too_big() {
        println!("Too big!");
    }

    fn win(tries: i8) {
        println!("You won in {} tries! Nice...", tries);
    }
}
