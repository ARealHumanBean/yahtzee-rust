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
    UpperScoreBonus(u32),
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
    pub fn yahtzee(player: &Player) -> Option<Score> {
        for i in 1..player.dice.len() {
            if player.dice[0] != player.dice[i] {
                return Some(Score::Yahtzee(0));
            }
        }
        // bonus points for already scoring yahtzee,
        // but only if the score was not for 0
        for player_score in player.scores.iter() {
            if let Score::Yahtzee(score_value) = player_score {
                if *score_value != 0 {
                    return Some(Score::Yahtzee(150));
                }
            }
        }

        Some(Score::Yahtzee(50))
    }

    /// Find an upper score for a die value
    ///
    /// The score value associated with the score enum is the die face
    /// multiplied by the count of them in the dice roll. So if the player has
    /// no dice for sixes, but two for fives, then the score would be
    /// `Score::Sixes(0)` and `Score::Fives(10)`.
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
    pub fn upper_score(player: &Player, die_face: u32) -> Option<Score> {
        let mut count = 0;
        for die in player.dice.iter() {
            if *die == die_face {
                count = count + 1;
            }
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

    /// returns the score bonus (35) if the total of upper scores is 63 or more.
    ///
    /// ```rust
    /// use yahtzee::player::Player;
    /// use yahtzee::score::Score;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.update_score(Score::Sixes(36));
    /// player.update_score(Score::Fives(30));
    /// if let Some(score) = Score::upper_score_bonus(&player) {
    ///     assert_eq!(score, Score::UpperScoreBonus(35));
    /// } else {
    ///     assert!(false);
    /// }
    /// ```
    pub fn upper_score_bonus(player: &Player) -> Option<Score> {
        let mut upper_score_total = 0;
        for score in player.scores.iter() {
            match score {
                Score::Aces(score)
                | Score::Twos(score)
                | Score::Threes(score)
                | Score::Fours(score)
                | Score::Fives(score)
                | Score::Sixes(score) => upper_score_total = upper_score_total + score,
                _ => (),
            }
        }
        if upper_score_total >= 63 {
            Some(Score::UpperScoreBonus(35))
        } else {
            None
        }
    }

    /// Find a large straight: a 5 dice sequence of consecutive values
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
    pub fn large_straight(player: &Player) -> Option<Score> {
        if player.scores.iter().any(|score| score.is_large_straight()) {
            return None;
        }

        let mut dice = player.dice;
        dice.sort();
        for i in 0..dice.len() - 1 {
            if dice[i] + 1 != dice[i + 1] {
                return Some(Score::LargeStraight(0));
            }
        }

        Some(Score::LargeStraight(40))
    }

    /// Find a small straight: a 4 dice sequence of consecutive values.
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
    pub fn small_straight(player: &Player) -> Option<Score> {
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

        Some(Score::SmallStraight(0))
    }

    /// find three of a kind and if found, return Score with dice sum as score value
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::score::Score;
    /// use yahtzee::player::Player;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.dice = [1,1,1,3,6];
    /// if let Some(score) = Score::three_of_a_kind(&player) {
    ///     assert_eq!(score, Score::ThreeOfAKind(12)); // 12 is the total of all die faces
    /// } else {
    ///     assert!(false);
    /// }
    pub fn three_of_a_kind(player: &Player) -> Option<Score> {
        if player.scores.iter().any(|score| score.is_three_of_a_kind()) {
            return None;
        }

        let mut score = 0;
        for die_face in 1..=6 {
            let mut count = 0;

            for die in player.dice.iter() {
                if *die == die_face {
                    count = count + 1;
                }
                if count >= 3 {
                    score = player.dice.iter().sum();
                }
            }
        }
        Some(Score::ThreeOfAKind(score))
    }

    /// find four of a kind and if found, return Score with dice sum as score value
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::score::Score;
    /// use yahtzee::player::Player;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.dice = [1,1,1,1,6];
    /// if let Some(score) = Score::four_of_a_kind(&player) {
    ///     assert_eq!(score, Score::FourOfAKind(10)); // 10 is the total of all die faces
    /// } else {
    ///     assert!(false);
    /// }
    pub fn four_of_a_kind(player: &Player) -> Option<Score> {
        if player.scores.iter().any(|score| score.is_four_of_a_kind()) {
            return None;
        }

        let mut score = 0;
        for die_face in 1..=6 {
            let mut count = 0;

            for die in player.dice.iter() {
                if *die == die_face {
                    count = count + 1;
                }
                if count >= 4 {
                    score = player.dice.iter().sum();
                }
            }
        }
        Some(Score::FourOfAKind(score))
    }

    /// Check for a full house in player and return score
    ///
    /// # Example
    /// ```rust
    /// use yahtzee::score::Score;
    /// use yahtzee::player::Player;
    ///
    /// let mut player = Player::new("test".to_owned());
    /// player.dice = [1,1,2,2,2];
    /// if let Some(score) = Score::full_house(&player) {
    ///     assert_eq!(score, Score::FullHouse(25));
    /// } else {
    ///     assert!(false);
    /// }
    /// ```
    pub fn full_house(player: &Player) -> Option<Score> {
        if player.scores.iter().any(|score| score.is_four_of_a_kind()) {
            return None;
        }

        let mut three_pair = false;
        let mut two_pair = false;
        for die_face in 1..=6 {
            let mut die_count = 0;
            for die in player.dice.iter() {
                if *die == die_face {
                    die_count = die_count + 1;
                }
            }

            // not possible to have a full house when there are more than 3
            // of the same die face
            if die_count > 3 {
                return Some(Score::FullHouse(0));
            }

            if !three_pair {
                three_pair = die_count == 3;
            }

            if !two_pair {
                two_pair = die_count == 2;
            }

            if three_pair && two_pair {
                return Some(Score::FullHouse(25));
            }
        }
        Some(Score::FullHouse(0))
    }

    /// Free score which allows the player to score for the sum of the dice
    pub fn chance(player: &Player) -> Option<Score> {
        if player.scores.iter().any(|score| score.is_chance()) {
            return None;
        }
        Some(Score::Chance(player.dice.iter().sum()))
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
            Score::UpperScoreBonus(score) => write!(f, "Upper Score Bonus! {}", score),
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
