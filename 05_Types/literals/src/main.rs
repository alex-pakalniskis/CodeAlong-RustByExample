fn main() {
    // suffixed literals
    let x = 2u8;
    let y = 4u32;
    let z = 6f32;

    // unsuffixed literals
    let i = 2;
    let f = 1.0;

    println!("size of x in bytes: {}", std::mem::size_of_val(&x));
    println!("size of y in bytes: {}", std::mem::size_of_val(&y));
    println!("size of z in bytes: {}", std::mem::size_of_val(&z));
    println!("size of i in bytes: {}", std::mem::size_of_val(&i));
    println!("size of f in bytes: {}", std::mem::size_of_val(&f));

}
