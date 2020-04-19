use std::{io, fmt};
extern crate rand;
use rand::distributions::{Distribution, Uniform};
/* Yahtzee flow
number of dice = 5
number of rerolls = 3
number of rounds = 13

*/

struct Player {
    name: String,
    score: i32
}

impl Player {
    fn new(name: String) -> Player {
        Player {name: name, score: 0}
    }
}

#[derive(Debug)]
struct Dice {
    dice: [u8; 5]
}

impl Dice {
    fn new(&mut self) -> Dice {
        Dice {dice: [0;5]}
    }

    fn roll(&mut self) {
        let mut rng = rand::thread_rng();
        let die_range = Uniform::from(1..7);

        for die in self.dice.iter_mut() {
            *die = die_range.sample(&mut rng) as u8;
        }
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.dice[0])
    }
}

struct Yahtzee {
    dice: u32,
    rerolls: u32,
    rounds: u32,
    players: Vec<Player>
}

fn introduction() {
    println!("Hello and welcome to YAHTZEE!!!")
}

fn get_players_count() -> u32 {
    loop {
        println!("How many people are playing?");
        let mut players = String::new();

        io::stdin().read_line(&mut players)
            .expect("Failed to read line");

        let players: u32 = match players.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return players
    }
}

fn main() {
    introduction();
    let players = get_players_count();

    if players == 1 {
        println!("Single Player game");
    } else {
        println!("{} Player game", players);
    }

    let mut yahtzee_dice: Dice = Dice {dice: [0;5]};
    println!("{:?}", yahtzee_dice);
    yahtzee_dice.roll();
    println!("{:?}", yahtzee_dice);
}