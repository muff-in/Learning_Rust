// another example

fn main() {
    let mut a = true;
    let mut b = true;
    a = a > b && b < a;
    b = !b;
    println!("a: {}", a);
    println!("b: {}", b);
}

// output: a: false
// b: false
