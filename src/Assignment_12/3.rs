// Accessing elements in a tuple

// Unlike array which uses [] for accessing an element, the value of the tuple can be accessed using the dot operator (.).
// tuple_name.indexvalue

fn main() {
    let data = ("Rust", 101, "86K", 'J'); // define a tuple

    println!("{} {} {} {}", data.0, data.1, data.2, data.3); // tuplename.indexvalue
}

// output: Rust 101 86K J
