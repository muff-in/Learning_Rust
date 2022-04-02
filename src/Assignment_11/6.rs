// Get Slice

// Slice is basically a portion of an array. It lets you refer to a subset of a contiguous memory location.
// But unlike an array, the size of the slice is not known at compile time.

// To declare a slice use the syntax:
// let slice_name: &[i32] = &arr[0..2]; // &[i32] is the type of the slice and [0..2] is the range of the slice.

fn main() {
    //define an array of size 4
    let arr: [i32; 4] = [1, 2, 3, 4];
    //define the slice
    let slice_array1: &[i32] = &arr; // &[i32] is a slice of i32
    let slice_array2: &[i32] = &arr[0..2]; //
                                           // print the slice of an array
    println!("Slice of an array: {:?}", slice_array1);
    println!("Slice of an array: {:?}", slice_array2);
}

// output: Slice of an array: [1, 2, 3, 4]
// Slice of an array: [1, 2]
