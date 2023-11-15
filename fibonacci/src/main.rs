fn main() {
    let mut input = String::new();

    let input: u128 = loop {
        println!("Quanti valori vuoi visualizzare");
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Input Error");

        match input.trim().parse() {
            Ok(num) => break num,
            _ => continue
        }
    };

    for i in 1..=input {
        print!("{} ", fibonacci(i));
    }

}



fn fibonacci(n: u128) -> u128 {
    let mut older: u128 = 0;
    let mut old: u128 = 1;
    let mut current: u128;
    if n==0 { return 0 }
    for _i in 2..=n {
        current = older + old;
        older = old;
        old = current;
    }
    old

}
