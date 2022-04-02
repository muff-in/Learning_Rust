
// Syntax 2:

// This syntax defines a tuple by specifying the type

let tuple_name : (&str, char, i32, f32) = ("string", 'a', 1, 2.0);

fn main() {
   
    let person_data : (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");  // define a tuple with type annotated

    println!("{:?}",person_data);
}

// output: ("Alex", 48, "35kg", "6ft")