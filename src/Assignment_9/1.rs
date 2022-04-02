// Integers
/* Variables of Integer data type hold whole number values.
There are two subtypes of integer data type in Rust, based on the number of bits occupied by a variable in memory. */

// letter u mean unsigned and i mean signed(-)

// explicitly define an integer

fn main() {
    let a: i8 = 108; // The 8-bit signed integer type
    let b: u8 = 38; // The 8-bit un-signed integer type
    let c: i16 = -102; // The 16-bit signed integer type
    let d: u16 = 839; // The 16-bit un-signed integer type
    let e: i32 = -283298; // The 32-bit signed integer type
    let f: u32 = 23232; // The 32-bit un-signed integer type
    let g: i64 = 2323232322; // The 64-bit signed integer type
    let h: u64 = 2323232; // The 64-bit un-signed integer type

    println!(
        "a:{}, b:{}, c:{}, d:{}, e:{}, f:{}, g:{}, h:{}",
        a, b, c, d, e, f, g, h
    );
}

// output: a:108, b:38, c:-102, d:839, e:-283298, f:23232, g:2323232322, h:2323232
