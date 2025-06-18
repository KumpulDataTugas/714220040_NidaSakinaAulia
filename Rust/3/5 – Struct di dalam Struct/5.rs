struct Alamat {
    kota: String,
    kode_pos: u32,
}

struct Pengguna {
    nama: String,
    alamat: Alamat,
}

fn main() {
    let pengguna = Pengguna {
        nama: String::from("Nida"),
        alamat: Alamat {
            kota: String::from("Bandung"),
            kode_pos: 40111,
        },
    };

    println!(
        "Nama: {}, Kota: {}, Kode Pos: {}",
        pengguna.nama, pengguna.alamat.kota, pengguna.alamat.kode_pos
    );
}
