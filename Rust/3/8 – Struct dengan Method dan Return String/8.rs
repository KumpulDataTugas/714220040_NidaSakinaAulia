struct Mobil {
    merk: String,
    tahun: u16,
}

impl Mobil {
    fn info(&self) -> String {
        format!("Mobil {} keluaran tahun {}", self.merk, self.tahun)
    }
}

fn main() {
    let m = Mobil {
        merk: String::from("Toyota"),
        tahun: 2020,
    };
    println!("{}", m.info());
}
