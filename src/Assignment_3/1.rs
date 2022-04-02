// we cannot directly print numbers or variables within the println!() macro, unlike other languages. We need a placeholder {}.
// The placeholder {} is used to indicate where the expression should be placed.

// Single placeholder
// A single placeholder is used when it is required to print a single value.

fn main() {
    println!("Number: {}", 1);
}

// output: Number: 1
