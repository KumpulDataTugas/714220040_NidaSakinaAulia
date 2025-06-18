fn main() {
    let mut x = 10;
    println!("Before: {}", x);

    x = 20; // OK karena mutable
    println!("After: {}", x);
}
