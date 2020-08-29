#![allow(dead_code)]

enum Specials {
    Crab,
    Octopus,
    Fish,
    Clam,
}
enum PoisonType {
    Acidic,
    Painful,
    Lethal,
}
enum Size {
    Big,
    Small,
}
enum Weapon {
    Claw(i32, Size),
    Posion(PoisonType),
    None,
}

struct SeaCreature {
    specials: Specials,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    // SeaCreatureのデータはスタックに入る
    let ferris = SeaCreature {
        // Stringu構造体もスタックに入るが
        // ヒープに入るデータの参照アドレスが一つ入る
        specials: Specials::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.specials {
        Specials::Crab => match ferris.weapon {
            Weapon::Claw(num_claws, size) => {
                let size_description = match size {
                    Size::Big => "big",
                    Size::Small => "small",
                };
                println!(
                    "ferris is a crab with {} {} claws",
                    num_claws, size_description
                );
            }
            _ => println!("ferris is a crab with some other weapon"),
        },
        _ => println!("ferris is some other animal"),
    }
}
