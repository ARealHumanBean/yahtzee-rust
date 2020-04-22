use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::{fmt, io, str};

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
    FullHouse,
    SmallStraight,
    LargeStraight,
    Chance(Dice),
    Yahtzee,
}

impl fmt::Display for Score {
    /// Display for different scores
    ///
    /// The score values are calculated in the Display. It might be a better idea to not do that.
    ///
    /// ```
    /// let twos = Score::Twos(2);
    /// assert_eq!(format!("{}", twos),"4");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Score::Aces(dice) => write!(f, "{}", dice),
            Score::Twos(dice) => write!(f, "{}", dice * 2),
            Score::Threes(dice) => write!(f, "{}", dice * 3),
            Score::Fours(dice) => write!(f, "{}", dice * 4),
            Score::Fives(dice) => write!(f, "{}", dice * 5),
            Score::Sixes(dice) => write!(f, "{}", dice * 6),
            Score::ThreeOfAKind(die) => write!(f, "{}", die * 3),
            Score::FourOfAKind(die) => write!(f, "{}", die * 4),
            Score::FullHouse => write!(f, "25"),
            Score::SmallStraight => write!(f, "30"),
            Score::LargeStraight => write!(f, "40"),
            Score::Chance(all_dice) => write!(f, "{}", all_dice.dice.iter().sum::<u8>()),
            Score::Yahtzee => write!(f, "50"),
        }
    }
}

// will you contain player data? yes?
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

fn round() -> u32 {
    let mut dice = Dice::new();
    let mut rolls = 1;
    let score = 0;

    'rounds: while rolls < 4 {
        println!("\n{}", dice);
        println!("Scores: ");
        println!("put a function here to check scores based on dice");
        if rolls < 3 {
            let is_reroll: bool = loop {
                println!("Do you want to reroll? true/false");
                match read_value() {
                    Ok(is_reroll) => break is_reroll,
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    },
                }
            };
            
            if !is_reroll {
                break 'rounds;
            }

            reroll(&mut dice);
        }
        rolls = rolls + 1;
    }
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
            },
        };
    };
    
    let mut players: Vec<Player> = Vec::new();
    if player_count == 1 {
        println!("Single Player game");
        players.push(Player::new(get_player_name()));
    } else {
        println!("{} Player game", player_count);
    }

    println!("{:?}", players);

    round();
}
