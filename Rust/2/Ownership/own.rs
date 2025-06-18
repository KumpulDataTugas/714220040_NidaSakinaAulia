fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // Ownership berpindah ke s2, s1 tidak bisa digunakan lagi

    // println!("{}", s1); // ❌ ERROR: value borrowed after move
    println!("{}", s2); // ✅ OK
}
