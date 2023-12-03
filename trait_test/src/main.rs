pub trait Area {
    fn get_area(&self) -> f64;
}

struct Quadrato {
    lato: f64,
}
impl Area for Quadrato {
    fn get_area(&self) -> f64 {
        return self.lato * self.lato;
    }
}

struct Rettangolo2 {
    base: f64,
    altezza: f64,
}

struct Rettangolo {
    base: f64,
    altezza: f64,
}
impl Area for Rettangolo {
    fn get_area(&self) -> f64 {
        return self.base * self.altezza;
    }
}

fn ciao(forma: &impl Area) -> i32 {
    return 0;
}

fn main() {
    let quadrato = Quadrato { lato: 5.0 };
    let rettangolo = Rettangolo {
        base: 4.0,
        altezza: 6.0,
    };
    let rettangolo2 = Rettangolo2 {
        base: 5.0,
        altezza: 8.0,
    };
    println!("Area del quadrato = {}", quadrato.get_area());
    println!("Area del rettangolo = {}", rettangolo.get_area());

    ciao(&rettangolo);
    // error = ciao(&rettangolo2);
}
