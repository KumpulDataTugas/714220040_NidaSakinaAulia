fn main() {
    {
        let s = String::from("hello");
        println!("{}", s);
    }
    // println!("{}", s); // ❌ ERROR: s out of scope
}
