use yahtzee::input::*;
use yahtzee::player::Player;

const NUM_ROUNDS: u8 = 13;

fn introduction() {
    println!("Hello and welcome to YAHTZEE!!!");
}

fn display_round<T: std::fmt::Display>(
    player: &Player,
    roll_counter: u32,
    possible_scores: &Vec<T>,
) {
    println!("{}'s Roll: {}", player.name, roll_counter);
    println!("Possible Scores:");
    for (i, possible_score) in possible_scores.iter().enumerate() {
        println!("\tScore {}: {}", (i + 1), possible_score)
    }
    println!("{}", player);
}

fn round(player: &mut Player) {
    player.roll_dice();
    'round: for roll_counter in 1..4 {
        let possible_scores = player.possible_scores();
        display_round(player, roll_counter, &possible_scores);
        if roll_counter < 3 {
            loop {
                println!("Enter the dice you'd like to reroll (Enter nothing to score)");
                match read_values::<u8>() {
                    Some(result) => match result {
                        Ok(dice) => {
                            player.reroll(dice);
                            continue 'round;
                        }
                        Err(error) => println!("error: {}", error),
                    },
                    None => break,
                }
            }
        }

        let score_index: usize = loop {
            println!("Select a possible score");
            match read_value() {
                Ok(score_index) if score_index > possible_scores.len() => {
                    println!("Your selection is too high")
                }
                Ok(score_index) if score_index < 1 => println!("Your selection is too low"),
                Ok(score_index) => break score_index,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            }
        };

        let score = possible_scores[score_index - 1];
        player.update_score(score);
        println!("{} scored! {}", player.name, score);
        break;
    }
}

fn main() {
    loop {
        introduction();

        let mut players: Vec<Player> = Vec::new();
        while let Some(player_name) = get_player_name() {
            players.push(Player::new(player_name));
            println!("Enter nothing when ready to continue.");
        }
        for round_incr in 1..=NUM_ROUNDS {
            for player in players.iter_mut() {
                print!("\n{}'s Round {}", player.name, round_incr);
                print!("  |  Current Score: {}\n", player.score);
                round(player);
            }
        }

        for player in players.iter_mut() {
            player.endgame();
            println!(
                "Thank you for playing yahtzee {}. Your score was: {}",
                player.name, player.score
            );
        }
    }
}
