use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    loop {
        println!("Inserisci numero");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Error while reading");
        match input.trim().parse::<i32>() {
            Ok(0) => break,
            Ok(i) => {
                v.push(i);
                let count = map.entry(i).or_insert(0);
                *count += 1;
            }
            _ => continue,
        };
    }
    v.sort();
    println!("{:?}", v);
    let idx = v.len() - 1;
    let mediano = v[idx / 2];
    println!("L'elemento mediano è {mediano}");

    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
}
