#![allow(dead_code)]

enum Specials {
    Crab,
    Octopus,
    Fish,
    Clam,
}

struct SeaCreature {
    specials: Specials,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let ferris = SeaCreature {
        specials: Specials::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.specials {
        // enum の値しか入ってこないのでelseはなくても大丈夫
        Specials::Crab => println!("{} is clab", ferris.name),
        Specials::Octopus => println!("{} is octopus", ferris.name),
        Specials::Fish => println!("{} is fish", ferris.name),
        Specials::Clam => println!("{} is clam", ferris.name),
    }
}
