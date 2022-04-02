// Tuples are heterogeneous sequences of elements, meaning, each element in a tuple can have a different data type.
// Just like arrays, tuples are of a fixed length.

//  tuple can be defined by writing let followed by the name of the tuple and then enclosing the values within the parenthesis.

// Syntax 1:

// let tuple_name = ("string", 'a', 1, 2.0);

fn main() {
    let person_data = ("Alex", 48, "35kg", "6ft"); //define a tuple

    println!("{:?}", person_data);
}

// here the tuple is defined with 4 elements of different types, string, char, integer and float.

// Above define tuple is without type annotation. However the compiler can infer the type of the elements in the tuple.

// output: ("Alex", 48, "35kg", "6ft")
