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
// traits and generics
// trait DisplayItem {
//     fn display(&self);
// }
// struct Inventory<T> {
//     item: T,
// }
// impl<T> DisplayItem for Inventory<T>
// where
//     T: std::fmt::Debug,
// {
//     fn display(&self) {
//         println!("Inventory item: {:?}", self.item);
//     }
// }

// fn main() {
//     let gold = Inventory { item: 100 };
//     gold.display();

//     let sword = Inventory { item: "sword" };
//     sword.display();
// }

// fn main() {
//     let map = String::from("old map");
//     let borrowed_map = &map;
//     let mut new_map = borrowed_map.to_string();
//     new_map.push_str(" to new map");
//     println!("{}", new_map);
// }

// fn main() {
//     let fruits = ["apple", "banana", "orange"];
//     for fruit in fruits.iter() {
//         if fruit == &"apple" {
//             println!("Found an apple!");
//         } else {
//             println!("{}", fruit);
//         }
//     }
// }
//
// Vector!
// fn main() {
//     let mut items = vec!["banana", "orange", "mango"];
//     let last_item = items.pop();
//     let first_item = items.remove(0);

//     let items_length = items.len();
//     let item_cap = items.capacity();
//     println!("{}", first_item);
// }
//Vector and closures
// fn main() {
//     let treasures = vec![100, 200, 300];
//     let new_treasures: Vec<i32> = treasures.iter().map(|x| x * 2).collect();
//     println!("doubled treasures : {:?}", new_treasures);
// }
//

// use std::collections::HashMap;
// // Hashmap
// fn main() {
//     let mut treasures: HashMap<&str, i32> = HashMap::new();
//     treasures.insert("Silver Coin", 100);
//     treasures.insert("Sword", 200);

//     if let Some(val) = treasures.get_mut("Silver Coin") {
//         *val += 5;
//     }

//     println!("{:?}", treasures);
// }

//Error handling
// fn main() {
//     let chest_result = match open_chest(true) {
//         None => println!("No treasure found!"),
//         Some(treasure) => println!("Treasure found: {}", treasure),
//     };
//     println!("{:?}", chest_result);
//     let door_result: String = match open_door(false) {
//         Ok(safe) => safe,
//         Err(err) => panic!("{}", err),
//     };
//     println!("{:?}", door_result);
// }
// //Option
// fn open_chest(is_empty: bool) -> Option<String> {
//     if is_empty {
//         None
//     } else {
//         Some("Gold".to_string())
//     }
// }

// fn open_door(is_danger: bool) -> Result<String, String> {
//     if is_danger {
//         Err("Danger!".to_string())
//     } else {
//         Ok("Door opened!".to_string())
//     }
// }
//Smart pointer
// fn main() {
//     let chest = Box::new(10);

//     let share_chest = Rc::new(RefCell::new(chest));

//     **share_chest.borrow_mut() += 10;
//     **share_chest.borrow_mut() += 5;
// }

// fn main() {
//     let chest = 10;

//     let share_chest = Rc::new(RefCell::new(chest));

//     *share_chest.borrow_mut() += 10;
//     *share_chest.borrow_mut() += 5;
//     println!("chest {}", chest);
// }

//Traits as a type
//
//
// trait Weapon {
//     fn attack(&self);
// }
// struct Sword;
// struct Bow;
// impl Weapon for Sword {
//     fn attack(&self) {
//         println!("swinging a sword!");
//     }
// }

// impl Weapon for Bow {
//     fn attack(&self) {
//         println!("shooting an arrow!");
//     }
// }
// fn main() {}

// trait Gear {
//     fn use_gear(&self);
// }

// struct Sword;
// struct Bow;
// struct Potion;
// struct Shield;

// impl Gear for Sword {
//     fn use_gear(&self) {
//         println!("swinging a sword!");
//     }
// }

// impl Gear for Bow {
//     fn use_gear(&self) {
//         println!("shooting an arrow!");
//     }
// }

// impl Gear for Potion {
//     fn use_gear(&self) {
//         println!("drinking a potion!");
//     }
// }

// impl Gear for Shield {
//     fn use_gear(&self) {
//         println!("blocking with a shield!");
//     }
// }

// fn use_gear<T: Gear>(item: T) {
//     item.use_gear();
// }
// fn main() {
//     let crab_sword = Sword;
//     let crab_bow = Bow;
//     let crab_potion = Potion;
//     let crab_shield = Shield;

//     use_gear(crab_sword);
//     use_gear(crab_bow);
//     use_gear(crab_potion);
//     use_gear(crab_shield);
// }

//Threading.
// fn main() {
//     let threaded = thread::spawn(|| println!("crabpy is cooking"));
//     threaded.join().unwrap(); //main thread wait the minors thread
// }
//
use std::{
    sync::{Arc, Mutex},
    thread,
};
fn main() {
    let gold = Arc::new(Mutex::new(10));

    let loot1 = thread::spawn({
        let gold_artifact = Arc::clone(&gold);
        move || {
            let mut gold = gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });
    let loot2 = thread::spawn({
        let gold_artifact = Arc::clone(&gold);
        move || {
            let mut gold = gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });

    let loot3 = thread::spawn({
        let gold_artifact = Arc::clone(&gold);
        move || {
            let mut gold = gold_artifact.lock().unwrap();
            *gold += 100;
        }
    });

    loot1.join().unwrap();
    loot2.join().unwrap();
    loot3.join().unwrap();

    println!("gold found :: {}", gold.lock().unwrap());
}
