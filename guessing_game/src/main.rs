use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Indovina il numero!");

    loop {
        println!("Inserisci un valore");
        // mut vuol dire che la variabile può essere riassegnata
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Errore durante la lettura input");
        println!("Hai inserito {guess}");

        // parse ritorna un enum Result e quindi si può controllare il valore di esso
        // Result può essere Ok oppure Err e Err(_) è un catchall
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Troppo piccolo"),
            Ordering::Greater => println!("Troppo grande"),
            Ordering::Equal => {
                println!("Hai indovinato!");
                break;
            },
        }
    }

}
