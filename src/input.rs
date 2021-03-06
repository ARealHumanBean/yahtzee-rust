use std::{io, str};

/// read value from and parse into type of passed in argument T
pub fn read_value<T: str::FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse()
}

/// read values split by a comma
pub fn read_values<T: str::FromStr>() -> Option<Result<Vec<T>, T::Err>> {
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) if input.trim() == "" => return None,
            Ok(_) => {
                return Some(
                    input
                        .trim()
                        .split_whitespace()
                        .map(|word| word.parse())
                        .collect(),
                )
            }
            Err(error) => println!("Error: {}", error),
        };
    }
}

/// Gets the players name from standard input
pub fn get_player_name() -> Option<String> {
    let mut player_name = String::new();
    loop {
        println!("What is your name?");

        match io::stdin().read_line(&mut player_name) {
            Ok(_) if player_name.trim() == "" => return None,
            Ok(_) => return Some(player_name.trim().to_owned()),
            Err(error) => println!("Error: {}", error),
        }
    }
}

pub fn get_dice<T: str::FromStr>() -> Option<Result<Vec<T>, T::Err>> {
    let mut dice = String::new();
    loop {
        match io::stdin().read_line(&mut dice) {
            Ok(_) if dice.trim() == "" => return None,
            Ok(_) => {
                return Some(
                    dice.trim()
                        .split_whitespace()
                        .map(|word| word.parse())
                        .collect(),
                )
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}
