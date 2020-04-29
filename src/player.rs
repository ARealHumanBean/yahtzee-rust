use crate::input::*;
use crate::score::Score;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub dice: [u32; 5],
    pub scores: Vec<Score>,
}

impl Player {
    /// Constructer for Player Struct
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::player::Player;
    ///
    /// let player = Player::new("test".to_owned());
    /// assert_eq!(player, Player{name: "test".to_owned(), score: 0, dice: [0; 5], scores: vec![]});
    /// ```
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            score: 0,
            dice: [0; 5],
            scores: vec![],
        }
    }

    /// Randomizes all the dice for a player
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::player::Player;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// let old_dice = player.dice;
    /// player.roll_dice();
    /// assert_ne!(player.dice, old_dice);
    /// ```
    pub fn roll_dice(&mut self) {
        let mut rng = rand::thread_rng();
        let die_range = Uniform::from(1..7);

        for die in self.dice.iter_mut() {
            *die = die_range.sample(&mut rng);
        }
    }

    /// randomizes a single die for a player
    ///
    /// ```rust
    /// use yahtzee::player::Player;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// let old_die = player.dice[0];
    /// player.roll_die(0);
    /// assert_ne!(player.dice[0], old_die);
    /// ```
    pub fn roll_die(&mut self, die: usize) {
        if die > self.dice.len() - 1 {
            println!("out of bounds");
            return;
        }

        let mut rng = rand::thread_rng();
        self.dice[die] = rng.gen_range(0, 6);
    }

    /// rerolls dice the user chooses to reroll
    pub fn reroll(&mut self) {
        println!("Enter the dice number of each dice you want to reroll seperated by commas(,)");
        let rerolls: Vec<u8> = read_values().unwrap();

        for die in rerolls {
            self.roll_die(die as usize - 1);
        }
    }

    /// returns the possible scores from the dice passed
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::player::Player;
    /// use yahtzee::score::Score;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.dice = [1,2,4,2,3];
    /// let scores = player.possible_scores();
    /// assert_eq!(scores, vec![Score::Aces(1), Score::Twos(4), Score::Threes(3), Score::Fours(4), Score::SmallStraight(30)]);
    /// ```
    pub fn possible_scores(&mut self) -> Vec<Score> {
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

    /// update player score and scores
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::score::Score;
    /// use yahtzee::player::Player;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.update_score(Score::Threes(9));
    /// assert_eq!(player.score, 9);
    /// assert_eq!(player.scores, vec![Score::Threes(9)]);
    /// ```
    pub fn update_score(&mut self, score: Score) {
        self.scores.push(score);
        self.score = self.score
            + match score {
                Score::Aces(score) => score,
                Score::Twos(score) => score,
                Score::Threes(score) => score,
                Score::Fours(score) => score,
                Score::Fives(score) => score,
                Score::Sixes(score) => score,
                Score::ThreeOfAKind(score) => score,
                Score::FourOfAKind(score) => score,
                Score::FullHouse(score) => score,
                Score::SmallStraight(score) => score,
                Score::LargeStraight(score) => score,
                Score::Chance(score) => score,
                Score::Yahtzee(score) => score,
            } as u32;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{player}'s Scores: [{scores}]\n{player}'s Dice: [{dice}]",
            player = self.name,
            scores = self
                .scores
                .iter()
                .map(|score| format!("{}", score))
                .collect::<Vec<String>>()
                .join(" "),
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
