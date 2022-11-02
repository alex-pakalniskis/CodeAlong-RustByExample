// fn main() {
//     for n in 1..101 {
//         if n % 15 == 0 {
//             println!("Alex Rocks");
//         } else if n % 3 == 0 {
//             println!("Alex");
//         } else if n % 5 == 0 {
//             println!("Rocks");
//         } else {
//             println!("{}", n);
//         }
//     }
// }

// fn main() {
//     for n in 1..=100 {
//         if n % 15 == 0 {
//             println!("Alex Rocks");
//         } else if n % 3 == 0 {
//             println!("Alex");
//         } else if n % 5 == 0 {
//             println!("Rocks");
//         } else {
//             println!("{}", n);
//         }
//     }
// }

// fn main() {
//     let names = vec!["Indexers", "Delegators", "Curators", "Developers"];

//     for name in names.iter() {
//         match name {
//             &"Ferris" => println!("There is a rustacean among us!"),
//             _ => println!("Hello {}", name),
//         }
//     }
//     println!("names: {:?}", names);
// }

// fn main() {
//     let names = vec!["Indexers", "Delegators", "Curators", "Developers"];

//     for name in names.into_iter() {
//         match name {
//             "Ferris" => println!("There is a rustacean among us!"),
//             _ => println!("Hello {}", name),
//         }
//     }
// }

fn main() {
    let mut names = vec!["Indexers", "Delegators", "Curators", "Developers", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            & mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("{:?}", names);
}
