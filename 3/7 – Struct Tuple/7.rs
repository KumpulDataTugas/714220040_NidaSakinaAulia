struct Warna(u8, u8, u8);

fn main() {
    let merah = Warna(255, 0, 0);
    println!("RGB: {}, {}, {}", merah.0, merah.1, merah.2);
}
