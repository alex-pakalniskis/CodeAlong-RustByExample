use std::convert::From;

// 1. 
// let my_str = "Are you ready for MIPs?";
// let my_string = String::from(my_str);

// 2. 
// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }

// fn main() {
    

//     let num = Number::from(30);
//     println!("My number is {:?}", num);
// }

// 3.
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

