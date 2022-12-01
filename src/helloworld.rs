// main function - in a vanilla Rust program, main fn is required to be able to run
fn main() {
    println!("Hello, world!");
    formatted() // as declared below
}

// non-main function which we can use in the main function
fn formatted() {
    let string = "Test";
    // {} format declares that we want to use a string variable in this println, which we declare after the string literal
    println!("Hello, {}!", string);
}
