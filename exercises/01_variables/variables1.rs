fn main() {
    // Added missing mut keyword to make x mutable
    let mut x = 6;
    println!("x has the value {} before override", x);

    x = 6;
    println!("x has the value {x}");
}
