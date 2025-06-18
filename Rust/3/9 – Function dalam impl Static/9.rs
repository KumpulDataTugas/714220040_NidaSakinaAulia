struct Lingkaran {
    jari: f64,
}

impl Lingkaran {
    fn baru(j: f64) -> Lingkaran {
        Lingkaran { jari: j }
    }
}

fn main() {
    let l = Lingkaran::baru(7.0);
    println!("Jari-jari: {}", l.jari);
}
