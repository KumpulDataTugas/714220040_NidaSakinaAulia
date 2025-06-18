fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Duplikat data, tidak pindah ownership

    println!("s1: {}, s2: {}", s1, s2);
}
