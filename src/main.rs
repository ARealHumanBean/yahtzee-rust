use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::{fmt, io};

/* Yahtzee flow
number of dice = 5
number of rerolls = 3
number of rounds = 13
*/

// will you contain player data? yes?
#[derive(Debug)]
struct Player {
    name: String,
    score: i32,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name: name,
            score: 0,
        }
    }
}

struct Dice {
    dice: [u8; 5],
}

impl Dice {
    // create new dice every round
    fn new() -> Dice {
        let mut new_dice = Dice { dice: [0; 5] };
        let mut rng = rand::thread_rng();
        let die_range = Uniform::from(1..7);

        for die in new_dice.dice.iter_mut() {
            *die = die_range.sample(&mut rng) as u8;
        }

        new_dice
    }

    fn roll(&mut self, die: usize) {
        if die > self.dice.len() - 1 {
            println!("out of bounds");
            return;
        }

        let mut rng = rand::thread_rng();
        self.dice[die] = rng.gen_range(0, 6);
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Dice: [{}]",
            self.dice
                .iter()
                .enumerate()
                .map(|(i, die)| format!("D{}:{}", i + 1, die))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

// Game data?
struct Yahtzee {
    rerolls: u32,
    rounds: u32,
    players: Vec<Player>,
}

fn introduction() {
    println!("Hello and welcome to YAHTZEE!!!")
}

fn get_players_count() -> u32 {
    let mut players = String::new();
    loop {
        println!("How many people are playing?");

        io::stdin()
            .read_line(&mut players)
            .expect("Failed to read line");

        let players: u32 = match players.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return players;
    }
}

fn get_player_name() -> String {
    let mut player_name = String::new();
    loop {
        println!("What is your name?");

        io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");

        return player_name.trim().to_owned();
    }
}

fn main() {
    introduction();
    let player_count = get_players_count();

    let mut players: Vec<Player> = Vec::new();

    if player_count == 1 {
        println!("Single Player game");
        players.push(Player::new(get_player_name()));
    } else {
        println!("{} Player game", player_count);
    }

    println!("{:?}", players);
    let mut yahtzee_dice = Dice::new();
    println!("{:}", yahtzee_dice);
    yahtzee_dice.roll(3);
    println!("{:}", yahtzee_dice);
}
