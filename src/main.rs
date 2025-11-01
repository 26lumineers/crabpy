// fn main() {
//     let x: i32 = 42;
//     let y: f64 = 3.14;

//     let z: i32 = x + y as i32;
//     println!("summery : {}", z);

//     let msg1 = String::from("Crabpy1");
//     let msg2 = "Crabpy2".to_string();
//     let msg3 = "Crabpy3";
//     let msg4 = format!("Point :: {},{}", x, y);
//     println!("{}", msg4);
// }
//
// fn main() {
//     let weather = "Sunny";

//     if weather == "Sunny" {
//         println!("It's sunny!");
//     } else if weather == "Rainy" {
//         println!("It's rainy!");
//     } else {
//         println!("It's not sunny!");
//     }
// }

// fn main() {
//     let monster = "Dragon";
//     match monster {
//         "Goblin" => println!("It's a Goblin!"),
//         "Dragon" => println!("It's a Dragon!"),
//         "Orc" => println!("It's an Orc!"),
//         "Troll" => println!("It's a Troll!"),
//         _ => println!("It's not a Goblin!"),
//     }
// }

// fn main() {
//     let mut wood: i32 = 0;

//     loop {
//         wood += 1;
//         println!("Crabpy gather wood {} ea", wood);

//         if wood == 10 {
//             println!("You have gathered enough wood!");
//             break;
//         }
//     }
// }

// fn main() {
//     let mut res = quest("hunting", 30);
//     println!("{}", res);

//     res = quest("fishing", 50);
//     println!("{}", res);
//     res = quest("cooking", 10);
//     println!("{}", res);
// }

// fn quest(msg: &str, min: i32) -> String {
//     format!("Craby successfully done {} in {} min", msg, min)
// }
//! ownership with clone --> memory is alloc more .
// fn main() {
//     let treasure = String::from("Gold");
//     let treasure_copy = treasure.clone();
// }

//! ownership
// fn main() {
//     let treasure = String::from("Gold");
//     let treasure_copy = treasure;
//     println!("{}", treasure); //can't because its already change ownership.
// }
//

// immutable borrowing.
// fn main() {
//     let treasure = String::from("Gold");
//     let treasure_copy = &treasure;

//     println!("{}", treasure);
//     println!("{}", treasure_copy);
// }
