use std::io;

fn main() {
    println!("Enter the name of your champion!");

    let mut input_name = String::new();

    io::stdin()
        .read_line(&mut input_name)
        .expect("Could not read the line please enter again");

    let new_hero = Hero {
        name: input_name,
        health: 60,
        mp: 30
    };

    println!(
        "Your hero's name is: {}",
        new_hero.get_name()
    );

    println!("Enter the name of your companion!");

    let mut comp_name = String::new();

    io::stdin()
        .read_line(&mut comp_name)
        .expect("Could not read the line");

    let mut companion = Hero {
        name: comp_name,
        ..new_hero
    };

    let health_2 = companion.health;

    // println!("{health_2}");
}


struct Hero {
    name: String,
    health: u32,
    mp: u32
}

impl Hero {
    fn get_name(&self) -> &String {
        &self.name
    }
}