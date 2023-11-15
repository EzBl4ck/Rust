use std::io;

fn main() {

    let option = loop {
        println!("Scegli la modalità di conversione");
        println!("0- Celsius to Fahrenheit");
        println!("1- Fahrenheit to Celsius");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed input");

        match input.trim().parse::<i32>() {
            Ok(0) => break 0,
            Ok(1) => break 1,
            _ => continue,
        };
    };

    println!("Hai scelto l'opzione {option}");
    let init_temperature = loop {
        println!("Inserisci un valore");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed input");

        match input.trim().parse::<f64>() {
            Ok(val) => break val,
            _ => continue,
        }
    };

    let result;
    if option == 0 {
        result = (init_temperature * (9.0/5.0)) + 32.0;
    } else {
        result = (init_temperature - 32.0) * (5.0/9.0);
    }

    println!("Il risultato è {result}");

}
