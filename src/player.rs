use std::fmt;
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use crate::input::*;
use crate::score::Score;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub dice: [u8; 5],
    pub scores: Vec<Score>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            score: 0,
            dice: [0; 5],
            scores: vec![],
        }
    }

    pub fn roll_dice(&mut self) {
        let mut rng = rand::thread_rng();
        let die_range = Uniform::from(1..7);

        for die in self.dice.iter_mut() {
            *die = die_range.sample(&mut rng) as u8;
        }
    }

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