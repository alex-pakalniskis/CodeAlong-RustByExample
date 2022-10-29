fn main() {
    let mut _mutable_integer = 7i32;
    println!("what is the value?: {}", _mutable_integer);

    {
        let _mutable_integer = _mutable_integer;
        // _mutable_integer = 50;
    }

    _mutable_integer = 3;

    println!("what is the value?: {}", _mutable_integer);
}
