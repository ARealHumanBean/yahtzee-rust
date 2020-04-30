use yahtzee::input::*;
use yahtzee::player::Player;

const NUM_ROUNDS: u8 = 13;

fn introduction() {
    println!("Hello and welcome to YAHTZEE!!!");
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
                player.roll_dice();

                for rolls in 1..4 {
                    print!("\n{}'s Round {}", player.name, round_incr);
                    print!("  |  Current Score: {}", player.score);
                    println!("  |  Roll: {}", rolls);

                    let possible_scores = player.possible_scores();
                    println!("Possible Scores:\n");
                    for (i, possible_score) in possible_scores.iter().enumerate() {
                        println!("\tScore {}: {} points", (i + 1), possible_score)
                    }

                    println!("{}", player);
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

                        if is_reroll {
                            player.reroll();
                            continue;
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

                    player.update_score(possible_scores[score_index - 1]);
                    break;
                }
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
