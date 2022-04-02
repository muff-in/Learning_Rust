// Another example

fn main() {
    let mut a = 1;
    let mut b = 2;
    a = a & b;
    a = a << 1;
    b = b >> 3;
    println!("a: {}", a);
    println!("b: {}", b);
}

// output: a: o
// b: 0

