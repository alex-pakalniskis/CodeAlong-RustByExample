fn main() {
    let elem = 7u8;

    // type annotation is redudant in this case because of inference
    let mut vec: Vec<u8> = Vec::new();

    vec.push(elem);

    println!("{:?}", vec);
}
