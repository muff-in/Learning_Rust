
// Calculate (a + b)^3
// (a+b)^3 =a^3 + b^3 + 3ab(a+b)

fn test() {
    let a = 2;
    let b = 2;
    let c = i32::pow(a, 3) + i32::pow(b, 3) + (3 * a * b * (a + b));
    println!("{}", c);
}

// output: 36

// The power of x^y is calculated as datatype :: pow(x,y). Here, the data type is the return value of the power function.
