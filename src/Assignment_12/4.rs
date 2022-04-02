// Other way to access the values in a tuple

// To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

fn main() {
    let data = ("Rust", 101, "86K", 'J'); // define a tuple
    let (w, x, y, z) = data; // destructure the tuple
    println!("{} {} {} {}", w, x, y, z); // print the values
}

// output: Rust 101 86K J
