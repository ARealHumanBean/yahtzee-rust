use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::{fmt, io, str};

const NUM_ROUNDS: u8 = 13;

/// Holds the different Score types a Yahtzee game can have
///
/// ```
/// let aces = Score::Aces = 2;
/// assert_eq!(aces, 2);
/// ```
enum Score {
    Aces(u8),
    Twos(u8),
    Threes(u8),
    Fours(u8),
    Fives(u8),
    Sixes(u8),
    ThreeOfAKind(u8),
    FourOfAKind(u8),
    FullHouse(u8),
    SmallStraight(u8),
    LargeStraight(u8),
    Chance(u8),
    Yahtzee(u8),
}

impl fmt::Display for Score {
    /// Display for different scores
    ///
    /// ```
    /// let twos = Score::Twos(4);
    /// assert_eq!(format!("{}", twos),"4");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Score::Aces(score) => write!(f, "Aces: \t{}", score),
            Score::Twos(score) => write!(f, "Twos: \t{}", score),
            Score::Threes(score) => write!(f, "Threes: {}", score),
            Score::Fours(score) => write!(f, "Fours: \t{}", score),
            Score::Fives(score) => write!(f, "Fives: \t{}", score),
            Score::Sixes(score) => write!(f, "Sixes: \t{}", score),
            Score::ThreeOfAKind(score) => write!(f, "Three of a Kind: {}", score),
            Score::FourOfAKind(score) => write!(f, "Four of a Kind: {}", score),
            Score::FullHouse(score) => write!(f, "Full House: {}", score),
            Score::SmallStraight(score) => write!(f, "Small Straight: {}", score),
            Score::LargeStraight(score) => write!(f, "Large Straight: {}", score),
            Score::Chance(score) => write!(f, "Chance: {}", score),
            Score::Yahtzee(score) => write!(f, "Yahtzee: {}", score),
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    score: u32,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name: name,
            score: 0,
        }
    }
}
#[derive(Copy, Clone)]
struct Dice {
    dice: [u8; 5],
}

impl Dice {
    /// create new dice every round
    /// ```
    /// use Dice::new();
    /// assert_eq!(1, 1);
    /// ```
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

    fn is_yahtzee(self) -> bool {
        for i in 1..self.dice.len() {
            if self.dice[0] != self.dice[i] {
                return false;
            }
        }
        true
    }

    fn upper_score(self, die_face: u8) -> Result<Score, String> {
        let mut count = 0;
        for die in self.dice.iter() {
            if *die == die_face {
                count = count + 1;
            }
        }
        if count == 0 {
            return Err(format!("count is 0 for die value of {}", die_face));
        }

        match die_face {
            1 => Ok(Score::Aces(count)),
            2 => Ok(Score::Twos(count * die_face)),
            3 => Ok(Score::Threes(count * die_face)),
            4 => Ok(Score::Fours(count * die_face)),
            5 => Ok(Score::Fives(count * die_face)),
            6 => Ok(Score::Sixes(count * die_face)),
            _ => Err(format!(
                "die_face value {} too large. Needs to be 1-6",
                die_face
            )),
        }
    }

    fn is_large_straight(self) -> bool {
        let mut dice = self.dice;
        dice.sort();
        for i in 0..dice.len() - 1 {
            if dice[i] + 1 != dice[i + 1] {
                return false;
            }
        }
        true
    }

    fn is_small_straight(self) -> bool {
        let mut dice = self.dice;
        dice.sort();
        let mut sequence_counter = 0;
        println!("{:?}", dice);
        for i in 0..dice.len() - 1 {
            if dice[i] + 1 == dice[i + 1] {
                sequence_counter = sequence_counter + 1;
            } else if dice[i] == dice[i + 1] {
                continue;
            } else {
                sequence_counter = 0;
            }

            if sequence_counter == 3 {
                return true;
            }
        }

        false
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

fn read_value<T: str::FromStr>() -> Result<T, T::Err> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse()
}

/// read values split by a comma
fn read_values<T: str::FromStr>() -> Result<Vec<T>, T::Err> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().split(",").map(|word| word.parse()).collect()
}

fn introduction() {
    println!("Hello and welcome to YAHTZEE!!!")
}

/// rerolls dice the user chooses to reroll
fn reroll(dice: &mut Dice) -> &mut Dice {
    println!("Enter the dice number of each dice you want to reroll seperated by commas(,)");
    let rerolls: Vec<u8> = read_values().unwrap();

    for die in rerolls {
        dice.roll(die as usize - 1);
    }

    dice
}

fn possible_scores(dice: Dice) -> Vec<Score> {
    let mut scores: Vec<Score> = vec![];

    for die_face in 1..=6 {
        match dice.upper_score(die_face) {
            Ok(score) => scores.push(score),
            Err(_) => (),
        };
    }

    if dice.is_small_straight() {
        scores.push(Score::SmallStraight(30));
    }

    if dice.is_large_straight() {
        scores.push(Score::LargeStraight(40));
    }

    if dice.is_yahtzee() {
        scores.push(Score::Yahtzee(50));
    }

    scores
}

fn round() -> u32 {
    let mut dice = Dice::new();
    let mut rolls = 1;
    let score = 0;

    'rounds: while rolls < 4 {
        let scores = possible_scores(dice);
        println!("\nScores: ");
        for score in scores {
            println!("\t{} points", score)
        }

        if rolls < 3 {
            let is_reroll: bool = loop {
                println!("\nRoll {}: {}", rolls, dice);
                println!("\nDo you want to reroll? true/false");
                match read_value() {
                    Ok(is_reroll) => break is_reroll,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                }
            };

            if !is_reroll {
                break 'rounds;
            }

            reroll(&mut dice);
        }
        rolls = rolls + 1;
    }

    println!("function to score here");
    score
}

/// Gets the players name from standard input
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

    let player_count: u32 = loop {
        println!("How many players?");
        match read_value() {
            Ok(num) => break num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
    };

    let mut players: Vec<Player> = Vec::new();
    if player_count == 1 {
        println!("Single Player game");
        players.push(Player::new(get_player_name()));
    } else {
        println!("{} Player game", player_count);
    }

    for round_incr in 1..=NUM_ROUNDS {
        for player in players.iter() {
            println!("\n{}'s Round {}", player.name, round_incr);
            println!("Current Score: {}", player.score);
            round();
        }
    }
}
