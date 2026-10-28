use std::io;

fn prompt() -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.to_ascii_lowercase().starts_with("y")
}

pub fn intro() {
    println!("Hello brave explorer! Want to embark on an adventure?");
    let answer = prompt();
    // If the player's answer was yes, print a message
    if answer == true {
        println!("Yay! Your aspirations will give you some very good karma later in your life.");
    // else, if the answer is no (or anything else!)
    // print a different, less friendly message
    } else {
        println!("Boo, you're such a buzzkill!");
    }
}
