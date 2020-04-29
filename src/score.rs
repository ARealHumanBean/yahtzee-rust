use crate::player::Player;
use derive_is_enum_variant::is_enum_variant;
use std::fmt;

/// Holds the different types of scores that are possible in a Yahtzee game.
///
/// # Example
/// ```rust
/// use yahtzee::score::Score;
///
/// let score_value = 50;
/// let yahtzee = Score::Yahtzee(score_value);
/// ```
#[derive(Debug, is_enum_variant, PartialEq, Clone, Copy)]
pub enum Score {
    Aces(u32),
    Twos(u32),
    Threes(u32),
    Fours(u32),
    Fives(u32),
    Sixes(u32),
    ThreeOfAKind(u32),
    FourOfAKind(u32),
    FullHouse(u32),
    SmallStraight(u32),
    LargeStraight(u32),
    Chance(u32),
    Yahtzee(u32),
}

/// Methods to check for valid scores
impl Score {
    /// Find yahtzee and return it if found in the dice
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::player::Player;
    /// use yahtzee::score::Score;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.dice = [6;5];
    /// if let Some(first_yahtzee) = Score::find_yahtzee(&player) {
    ///     assert_eq!(first_yahtzee, Score::Yahtzee(50));
    /// } else {
    ///     assert!(false);
    /// }
    /// ```
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

        Some(Score::Yahtzee(50))
    }

    /// Find an upper score for a die value
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::player::Player;
    /// use yahtzee::score::Score;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.dice = [1,1,2,2,2];
    /// if let Some(aces) = Score::find_upper_score(&player, 1) {
    ///     assert_eq!(aces, Score::Aces(2));
    /// } else {
    ///     assert!(false);
    /// }
    pub fn find_upper_score(player: &Player, die_face: u32) -> Option<Score> {
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
            1 => {
                if player.scores.iter().any(|score| score.is_aces()) {
                    None
                } else {
                    Some(Score::Aces(count))
                }
            }
            2 => {
                if player.scores.iter().any(|score| score.is_twos()) {
                    None
                } else {
                    Some(Score::Twos(count * die_face))
                }
            }
            3 => {
                if player.scores.iter().any(|score| score.is_threes()) {
                    None
                } else {
                    Some(Score::Threes(count * die_face))
                }
            }
            4 => {
                if player.scores.iter().any(|score| score.is_fours()) {
                    None
                } else {
                    Some(Score::Fours(count * die_face))
                }
            }
            5 => {
                if player.scores.iter().any(|score| score.is_fives()) {
                    None
                } else {
                    Some(Score::Fives(count * die_face))
                }
            }
            6 => {
                if player.scores.iter().any(|score| score.is_sixes()) {
                    None
                } else {
                    Some(Score::Sixes(count * die_face))
                }
            }
            _ => None,
        }
    }

    /// Find a large straight from dice
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::player::Player;
    /// use yahtzee::score::Score;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.dice = [1,2,3,4,5];
    /// if let Some(large_straight) = Score::find_large_straight(&player) {
    ///     assert_eq!(large_straight, Score::LargeStraight(40));
    /// } else {
    ///     assert!(false);
    /// }
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

    /// Find a small straight from dice
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::player::Player;
    /// use yahtzee::score::Score;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.dice = [3,2,4,1,6];
    /// if let Some(small_straight) = Score::find_small_straight(&player) {
    ///     assert_eq!(small_straight, Score::SmallStraight(30));
    /// } else {
    ///     assert!(false);
    /// }
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
    /// # Example
    /// ```rust
    /// use yahtzee::score::Score;
    /// let twos = Score::Twos(4);
    /// assert_eq!(format!("{}", twos),"Twos: 4");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Score::Aces(score) => write!(f, "Aces: {}", score),
            Score::Twos(score) => write!(f, "Twos: {}", score),
            Score::Threes(score) => write!(f, "Threes: {}", score),
            Score::Fours(score) => write!(f, "Fours: {}", score),
            Score::Fives(score) => write!(f, "Fives: {}", score),
            Score::Sixes(score) => write!(f, "Sixes: {}", score),
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
