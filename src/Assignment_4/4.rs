// eprintln!()
// The eprintln!() macro displays the output as an error and adds a new line.

fn main() {
    eprintln!("Rust is a statically-typed");
    eprintln!("programming language");
}

// output: Rust is a statically-typed
// programming language [error]

// eprint!() and eprintln!() come in handy when we want to indicate to the user that an error condition has occurred.
