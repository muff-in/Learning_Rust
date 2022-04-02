// lets see how not to declare a local variable.

fn main() {
    let x = 5; // declare a local variable, x, and initialize it with a value of 5.
}
println!("Number: {}", x);

// output: error: because x is decalred in the another block of code, it is not accessible here.
