mod utils; // mengimpor module `utils.rs`

fn main() {
    utils::say_hello("Nida");
    let result = utils::multiply(5, 3);
    println!("Hasil perkalian: {}", result);
}
