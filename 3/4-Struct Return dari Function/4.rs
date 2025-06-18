struct Koordinat {
    x: i32,
    y: i32,
}

fn buat_koordinat(x: i32, y: i32) -> Koordinat {
    Koordinat { x, y }
}

fn main() {
    let k = buat_koordinat(3, 4);
    println!("Koordinat: ({}, {})", k.x, k.y);
}
