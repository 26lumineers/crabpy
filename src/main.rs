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
//     let borrowed_treasure = &treasure;

//     println!("original : {}", treasure);
//     println!("borrowed_treasure : {}", borrowed_treasure);
// }
//
// Mutable borrowing, cant borrow two at the same time.
// fn main() {
//     let mut treasure = String::from("Golden coins");
//     let borrowed_treasure = &mut treasure;
//     let borrowed_treasure_2 = &mut treasure;
//     borrowed_treasure.push_str(" and silver coins.");

//     println!("borrowed_treasure : {}", borrowed_treasure);
//     println!("original : {}", treasure);
// }

// Mutable borrowing
// fn main() {
//     let mut treasure = String::from("Golden coins");
//     let borrowed_treasure = &mut treasure;
//     borrowed_treasure.push_str(" and silver coins.");

//     println!("borrowed_treasure : {}", borrowed_treasure);
//     println!("original : {}", treasure);
// }
// fn main() {
//     let mut treasure = String::from("Gold");

//     let friend1 = &treasure;
//     let friend2 = &treasure;

//     println!("friend 1 : {}", friend1);
//     println!("friend 2 : {}", friend2);

//     let trusted_friend = &mut treasure;
//     trusted_friend.push_str(" and silver coins.");

//     println!("trusted friend : {}", trusted_friend)
// }

// fn main() {
//     let map1 = "defense of the ancients.";
//     let map2 = "leaguge of legends.";
//     println!("the longest_map is {}", longest_map(map1, map2))
// }
// //make life time the be the same
// fn longest_map<'b>(map1: &'b str, map2: &'b str) -> &'b str {
//     if map1.len() > map2.len() { map1 } else { map2 }
// }
//struct
// struct Crabpy {
//     name: String,
//     hp: u8, //scale is 100
//     level: u8,
// }
// impl Crabpy {
//     fn new(name: String, hp: u8, level: u8) -> Self {
//         Crabpy { name, hp, level }
//     }
//     fn take_damage(&mut self, damage: u8) {
//         self.hp = self.hp.saturating_sub(damage);
//         println!("take incoming dmg {} ,now hp is {}", damage, self.hp)
//     }
//     fn healing(&mut self, amount: u8) {
//         if self.hp + amount > 100 {
//             self.hp = 100;
//         } else {
//             self.hp += amount;
//         }
//         println!("healing {} ,now hp is {}", amount, self.hp)
//     }
// }
// fn main() {
//     let mut crab = Crabpy::new("film".to_string(), 100, 20);
//     crab.take_damage(50);
//     crab.take_damage(90);
//     crab.healing(90);
// }

//Enum
// enum CrabpyState {
//     Resting,
//     Fighting,
//     Defending,
//     Collecting(u32),
// }
// impl CrabpyState {
//     fn state_respresent(&self) {
//         match self {
//             CrabpyState::Resting => println!("crabpy is resting"),
//             CrabpyState::Fighting => println!("crabpy is fighting"),
//             CrabpyState::Defending => println!("crabpy is defending"),
//             CrabpyState::Collecting(amount) => println!("crabpy is collecting {}", amount),
//         }
//     }
// }
// fn main() {
//     // let current_state = CrabpyState::Resting;

//     // match current_state {
//     //     CrabpyState::Resting => println!("crabpy is resting"),
//     //     CrabpyState::Fighting => println!("crabpy is fighting"),
//     //     CrabpyState::Defending => println!("crabpy is defending"),
//     //     CrabpyState::Collecting(amount) => println!("crabpy is collecting {}", amount),
//     // }
//     let fighting = CrabpyState::Fighting;
//     let defending = CrabpyState::Defending;
//     let resting = CrabpyState::Resting;
//     let collecting = CrabpyState::Collecting(100);

//     collecting.state_respresent();
//     fighting.state_respresent();
//     defending.state_respresent();
//     resting.state_respresent();
// }
//
// traits
trait Attack {
    fn execute(&self);
}

fn main() {}
