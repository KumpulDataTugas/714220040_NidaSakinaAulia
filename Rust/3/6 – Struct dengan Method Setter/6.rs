struct Akun {
    nama: String,
    saldo: f64,
}

impl Akun {
    fn set_saldo(&mut self, s: f64) {
        self.saldo = s;
    }
}

fn main() {
    let mut akun = Akun {
        nama: String::from("Nida"),
        saldo: 100.0,
    };
    akun.set_saldo(250.0);
    println!("Saldo: {}", akun.saldo);
}
