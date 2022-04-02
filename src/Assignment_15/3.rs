// Type Casting
// Type casting is when you convert the data type of the variable to some other data type.

// In Rust, typecasting is done using the as keyword followed by the desired data type of the variable or value.

// operand as datatype

fn main() {
    let a = 15;
    let b = (a as f64) / 2.0;
    println!("a: {}", a);
    println!("b: {}", b);
}

// output: a:15
// b:7.5

// NOTE: What data types can be type casted?

// Integer can be type casted to floating-point and vice versa.
// Integer can be typecasted to String

// What data types cannot be type casted?

// String (&str) or character cannot be type casted to the data type of type integer or float.
// Character cannot be type casted to String type and vice versa.
