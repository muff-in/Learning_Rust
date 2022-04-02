// Positional Arguments
/*Positional arguments specify the positions of the values in a sentence.

Each value is assigned a number based on the order of occurrence. The first value is assigned 0, the next is assigned 1, and so on and so forth.
The placeholder takes an integer positive number (greater than or equal to 0)
which indicates the value to be inserted in the placeholder is to be picked from the list of values in a given order. */

fn main() {
    println!(
        "Start using {0}. {0} is safe and c is {1}.",
        "Rust", "Pretty shit"
    );
}

// output: Start using Rust. Rust is safe and c is Pretty shit.
