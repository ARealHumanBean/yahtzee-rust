use std::io;
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
    fn new(name: String) -> Player{
        Player{name: name, score: 0}
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
    println!("{}", get_players_count());
}
