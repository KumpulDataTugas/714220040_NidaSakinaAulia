struct Persegi {
    sisi: f64,
}

impl Persegi {
    fn luas(&self) -> f64 {
        self.sisi * self.sisi
    }
}

fn main() {
    let p = Persegi { sisi: 5.0 };
    println!("Luas: {}", p.luas());
}
