#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Solidier,
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have a lot less money"),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Solidier => println!("Soldiers fight!"),
    }
}