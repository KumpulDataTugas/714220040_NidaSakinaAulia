struct Suhu {
    celcius: f64,
}

impl Suhu {
    fn ke_fahrenheit(&self) -> f64 {
        self.celcius * 1.8 + 32.0
    }
}

fn main() {
    let suhu = Suhu { celcius: 30.0 };
    println!("Fahrenheit: {}", suhu.ke_fahrenheit());
}
