// Placeholder for a Debug Trait

// It is possible to display multiple values using a single placeholder with the help of the debug trait (a colon followed by a question mark {:?}).

// This prevents having to write placeholders for each value.
// You can use a debug trait and write as many values as desired within the parentheses.

fn main() {
    println!(
        "{:?}",
        (
            "Rust is a tatically-typed",
            "programming language",
            "Rust is safe and fast"
        )
    );
}

//  output: (Rust is a tatically-typed, programming language, Rust is safe and fast)
