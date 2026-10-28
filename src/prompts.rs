use crate::player::Class;
use std::io;

fn prompt() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_lowercase()
}

pub fn intro() {
    println!("Hello brave explorer! Want to embark on an adventure? y/n");
    match prompt().as_str() {
        "y" => {
            println!("Yay! Your aspirations will give you some very good karma later in your life.")
        }

        "n" => println!("Boo, you're such a buzzkill!"),
        _ => {
            println!("Not a valid choice. Try again");
            intro()
        }
    }
}

pub fn class_choice() -> Class {
    println!("Now it's time to choose your class");
    println!("What do you choose?");
    println!("(S)wordsman, (W)izard or (H)ealer");

    match prompt().as_str() {
        "s" => Class::Swordsman,
        "w" => Class::Wizard,
        "h" => Class::Healer,
        _ => {
            println!("Not a valid choice. Try again");
            class_choice()
        }
    }
}

pub fn name() -> String {
    println!("It's time to choose a name");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
