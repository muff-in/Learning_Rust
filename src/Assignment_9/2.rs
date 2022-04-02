
// implicitly define integers

fn main() {
    let a = 108; // The 8-bit signed integer type
    let b = 38; // The 8-bit un-signed integer type
    let c = -102; // The 16-bit signed integer type
    let d = 839; // The 16-bit un-signed integer type
    let e = -283298; // The 32-bit signed integer type
    let f = 23232; // The 32-bit un-signed integer type
    let g = 2323232322; // The 64-bit signed integer type
    let h = 2323232; // The 64-bit un-signed integer type

    println!(
        "a:{}, b:{}, c:{}, d:{}, e:{}, f:{}, g:{}, h:{}",
        a, b, c, d, e, f, g, h
    );
}

// output: a:108, b:38, c:-102, d:839, e:-283298, f:23232, g:2323232322, h:2323232

// you don't have to explicitly define the data type, Rust will infer it for you.
