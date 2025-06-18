// This is your existing function
fn no_dangle() -> String {
    let s = String::from("hello");
    // ownership dikembalikan, bukan referensinya
    s
}

// Add the main function here
fn main() {
    // You can call your no_dangle function from main, for example:
    let result = no_dangle();
    println!("{}", result); // This will print "hello"
}