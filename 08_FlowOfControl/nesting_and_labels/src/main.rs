// #![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        loop {
            println!("Entered the inner loop");
            

            break 'outer;
        }
    }
    println!("Exited the outer loop");
}
