use derive_is_enum_variant;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::{fmt, io, str};

const NUM_ROUNDS: u8 = 13;

/// Holds the different Score types a Yahtzee game can have
///
/// ```
/// let aces = Score::Aces(2);
/// assert_eq!(aces, 2);
/// ```
#[derive(Debug, derive_is_enum_variant::is_enum_variant)]
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

impl Score {
    fn find_yahtzee(player: &Player) -> Option<Score> {
        for i in 1..player.dice.len() {
            if player.dice[0] != player.dice[i] {
                return None;
            }
        }
        // bonus points for already scoring yahtzee
        if player.scores.iter().any(|score| score.is_yahtzee()) {
            return Some(Score::Yahtzee(150));
        }

        return Some(Score::Yahtzee(50));
    }

    fn find_upper_score(player: &Player, die_face: u8) -> Option<Score> {
        let mut count = 0;
        for die in player.dice.iter() {
            if *die == die_face {
                count = count + 1;
            }
        }
        if count == 0 {
            return None;
        }

        match die_face {
            1 => Some(Score::Aces(count)),
            2 => Some(Score::Twos(count * die_face)),
            3 => Some(Score::Threes(count * die_face)),
            4 => Some(Score::Fours(count * die_face)),
            5 => Some(Score::Fives(count * die_face)),
            6 => Some(Score::Sixes(count * die_face)),
            _ => None,
        }
    }

    fn find_large_straight(player: &Player) -> Option<Score> {
        if player.scores.iter().any(|score| score.is_large_straight()) {
            return None;
        }

        let mut dice = player.dice;
        dice.sort();
        for i in 0..dice.len() - 1 {
            if dice[i] + 1 != dice[i + 1] {
                return None;
            }
        }

        Some(Score::LargeStraight(40))
    }

    fn find_small_straight(player: &Player) -> Option<Score> {
        if player.scores.iter().any(|score| score.is_small_straight()) {
            return None;
        }

        let mut dice = player.dice;
        dice.sort();
        let mut comparison_correct_count = 0; // what's a good name for you?
        for i in 0..dice.len() - 1 {
            if dice[i] + 1 == dice[i + 1] {
                comparison_correct_count = comparison_correct_count + 1;
            } else if dice[i] == dice[i + 1] {
                continue;
            } else {
                comparison_correct_count = 0;
            }

            if comparison_correct_count == 3 {
                return Some(Score::SmallStraight(30));
            }
        }

        None
    }
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
    dice: [u8; 5],
    scores: Vec<Score>,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name: name,
            score: 0,
            dice: [0; 5],
            scores: vec![],
        }
    }

    fn roll_dice(&mut self) {
        let mut rng = rand::thread_rng();
        let die_range = Uniform::from(1..7);

        for die in self.dice.iter_mut() {
            *die = die_range.sample(&mut rng) as u8;
        }
    }

    fn roll_die(&mut self, die: usize) {
        if die > self.dice.len() - 1 {
            println!("out of bounds");
            return;
        }

        let mut rng = rand::thread_rng();
        self.dice[die] = rng.gen_range(0, 6);
    }

    fn show_dice(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    /// rerolls dice the user chooses to reroll
    fn reroll(&mut self) {
        println!("Enter the dice number of each dice you want to reroll seperated by commas(,)");
        let rerolls: Vec<u8> = read_values().unwrap();

        for die in rerolls {
            self.roll_die(die as usize - 1);
        }
    }

    fn possible_scores(&mut self) -> Vec<Score> {
        let mut scores: Vec<Score> = vec![];

        for die_face in 1..=6 {
            if let Some(upper_score) = Score::find_upper_score(self, die_face) {
                scores.push(upper_score);
            };
        }

        if let Some(small_straight) = Score::find_small_straight(self) {
            scores.push(small_straight);
        }

        if let Some(large_straight) = Score::find_large_straight(self) {
            scores.push(large_straight);
        }

        if let Some(yahtzee) = Score::find_yahtzee(self) {
            scores.push(yahtzee);
        }

        scores
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{player}'s Scores:\n{scores}\n{player}'s Dice: [{dice}]",
            player = self.name,
            scores = self
                .scores
                .iter()
                .map(|score| format!("{}", score))
                .collect::<Vec<String>>()
                .join("\n"),
            dice = self
                .dice
                .iter()
                .enumerate()
                .map(|(i, die)| format!("D{}:{}", i + 1, die))
                .collect::<Vec<String>>()
                .join(" "),
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

    // let player_count: u32 = loop {
    //     println!("How many players?");
    //     match read_value() {
    //         Ok(num) => break num,
    //         Err(err) => {
    //             println!("{}", err);
    //             continue;
    //         }
    //     };
    // };

    // let mut players: Vec<Player> = Vec::new();
    // for i in 0..player_count {
    //     println!("Player: {}", i);
    //     let player = Player::new(get_player_name());
    //     players.push(player);
    // }

    let mut player = Player::new(get_player_name());
    for round_incr in 1..=NUM_ROUNDS {
        print!("\n{}'s Round {}", player.name, round_incr);
        println!("  |  Current Score: {}", player.score);
        player.roll_dice();
        let mut rolls = 1;
        let score = 0;

        'rounds: while rolls < 4 {
            let possible_scores = player.possible_scores();
            println!("{}\n", player);

            println!("Possible Scores:\n");
            for possible_score in possible_scores {
                println!("\t{} points", possible_score)
            }

            if rolls < 3 {
                let is_reroll: bool = loop {
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

                player.reroll();
            }
            rolls = rolls + 1;
        }

        println!("function to score here");
    }
}
