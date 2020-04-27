use derive_is_enum_variant::is_enum_variant;

use crate::player::Player;
use std::fmt;

#[derive(Debug, is_enum_variant)]
pub enum Score {
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
    pub fn find_yahtzee(player: &Player) -> Option<Score> {
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

    pub fn find_upper_score(player: &Player, die_face: u8) -> Option<Score> {
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

    pub fn find_large_straight(player: &Player) -> Option<Score> {
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

    pub fn find_small_straight(player: &Player) -> Option<Score> {
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
    /// use yahtzee::score::Score;
    /// let twos = Score::Twos(4);
    /// assert_eq!(format!("{}", twos),"Twos: \t4");
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