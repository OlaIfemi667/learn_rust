use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Devinez le nombre!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("S'il vous plait rentrer le nombre");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Echec lors de la lecture de l'entrée utilisateur");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Vous avez entré: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Trop petit!"),
            Ordering::Greater => println!("Trop grand!"),
            Ordering::Equal => {
                println!("Gagnééé!");
                break;
            }
        }
    }
} 
