#![allow(dead_code)]
#![allow(unused_variables)]

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add, 
    Substract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let x = Operations::Add;
}