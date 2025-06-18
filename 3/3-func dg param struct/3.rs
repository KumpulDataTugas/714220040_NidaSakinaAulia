struct Buku {
    judul: String,
    tahun: u16,
}

fn info_buku(b: &Buku) {
    println!("Judul: {}, Tahun: {}", b.judul, b.tahun);
}

fn main() {
    let b = Buku {
        judul: String::from("Rust Programming"),
        tahun: 2023,
    };
    info_buku(&b);
}
