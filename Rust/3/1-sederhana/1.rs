struct Mahasiswa {
    nama: String,
    npm: u32,
}

fn cetak_data(mhs: &Mahasiswa) {
    println!("Nama: {}, NPM: {}", mhs.nama, mhs.npm);
}

fn main() {
    let mhs1 = Mahasiswa {
        nama: String::from("Nida"),
        npm: 714220040,
    };
    cetak_data(&mhs1);
}
