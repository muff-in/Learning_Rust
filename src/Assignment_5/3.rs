// Mutable variables

// as mentioned above, variables are immutable by default. However, you can make them mutable by using the mut keyword.

fn main() {
    let mut language = "Rust"; // define a mutable variable using the mut keyword.
    println!("Language: {}", language); // print the variable
    language = "Java"; // update the variable
    println!("Language: {}", language); // print the updated value of variable
}

// output: Language: Rust
// Language: Java
