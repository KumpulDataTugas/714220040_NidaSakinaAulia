fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // passing reference (borrow)

    println!("Length: {}", len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
