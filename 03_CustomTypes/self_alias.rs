#![allow(dead_code)]
#![allow(unused_variables)]

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let test = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    println!("{:?}", test.run(4, 92));
}