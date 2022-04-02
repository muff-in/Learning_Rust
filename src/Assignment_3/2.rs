// Multiple Placeholders
// We can use multiple placeholders within the println!() macro. The number of placeholders is the number of values that will be printed.

fn main() {
    println!(
        "{} is a {} programming language",
        "Rust", "statically-typed"
    );
}

// output: Rust is a statically-typed programming language
