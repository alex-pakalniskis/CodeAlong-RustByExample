fn main() {
    fizzbuzz_to(100);
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn is_divisible_by(left_hand_side: u32, right_hand_side: u32) -> bool {
    if right_hand_side == 0 {
        return false;
    }

    left_hand_side % right_hand_side == 0
}