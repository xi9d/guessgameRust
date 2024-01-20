use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("This is a guessing game");
    println!("Try and beat the game, goodluck!!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
 loop{
    let mut guess = String::new();
    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to get value!");
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("The guess value is {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("To high"),
        Ordering::Equal => {
            println!("You win");
            break;
        }
    }
}
}