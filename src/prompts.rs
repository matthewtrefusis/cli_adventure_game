use crate::player::Class;
use std::io::{self, Write};

pub fn read_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().ok()?;
    let mut input = String::new();
    if io::stdin().read_line(&mut input).ok()? == 0 {
        return None;
    }
    Some(input.trim().to_lowercase())
}

pub fn class_choice() -> Class {
    loop {
        println!("Choose a class: [s]wordsman, [w]izard, [h]ealer");
        match read_line("> ").as_deref() {
            Some("s") | Some("swordsman") => return Class::Swordsman,
            Some("w") | Some("wizard") => return Class::Wizard,
            Some("h") | Some("healer") => return Class::Healer,
            _ => println!("Please choose swordsman, wizard, or healer."),
        }
    }
}

pub fn name() -> String {
    loop {
        print!("Character name: ");
        io::stdout().flush().ok();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            let name = input.trim().to_string();
            let name = name.trim().to_string();
            if !name.is_empty() && name.len() <= 24 {
                return name;
            }
        }
        println!("Enter a name between 1 and 24 characters.");
    }
}
